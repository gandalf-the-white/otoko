# FreeBSD Incident Analysis Report

**Report ID:** 1e9a2e14-5f4f-46b5-9e42-0c86dc6bfb55
**Generated at:** 2026-10-17T14:43:00.550270+00:00
**Schema version:** 1.0
**Incident count:** 1
**Summary source:** Ollama
# FreeBSD Incident Analysis Report

## Executive Summary

Confirmed brute-force SSH credential attack against the FreeBSD host. Three rapid password failures for user 'spike' from 10.0.0.52 were immediately followed by a successful login from the same source. Within approximately two minutes, the authenticated user executed a privileged `sudo /usr/bin/id` reconnaissance command. An active TTY session (pts/0) from 10.0.0.52 was confirmed at observation time, indicating the foothold may still be live. Severity: High.

## Key Findings
- Three consecutive failed SSH password attempts for user 'spike' from 10.0.0.52 (ports 50100–50102) observed within 13 seconds.
- Successful SSH password authentication for the same user from the same source IP (port 50110) approximately 3 minutes after the failures.
- Privileged command `sudo /usr/bin/id` executed by 'spike' roughly 2 minutes after login success.
- Active session confirmed: `spike pts/0 10.0.0.52` from get_recent_logins (exit status 0).
- Root filesystem (/dev/ada0p2) at 20% utilisation (100G total, 20G used) per get_socket_list (exit status 0).

## Operational Impact
At minimum, an unauthorised party (or an insider with prior credential knowledge) gained password-based SSH access to the host and performed initial privilege-level reconnaissance. The confirmed active TTY suggests the session may not have been terminated. The exact scope of post-authentication activity beyond the single logged `sudo id` command is undetermined.

## Priorities
- Terminate the active 'spike' session on pts/0 and lock the account.
- Block traffic from 10.0.0.52 at the host firewall (pf) or network perimeter.
- Rotate all credentials associated with the 'spike' account and audit for reuse elsewhere.

## Incident 1: Brute-force SSH login followed by privileged reconnaissance

**Severity:** High

### Evidence
- `Failed password for spike from 10.0.0.52 port 50100 ssh2`
- `Failed password for spike from 10.0.0.52 port 50101 ssh2`
- `Failed password for spike from 10.0.0.52 port 50102 ssh2`
- `Accepted password for spike from 10.0.0.52 port 50110 ssh2`
- `spike : COMMAND=/usr/bin/id`

### System Observations
- **get_socket_list**: Success
```text
/dev/ada0p2  100G  20G  80G  20% /
```
- **get_recent_logins**: Success
```text
spike pts/0 10.0.0.52
```

### Probable Causes
- The 'spike' account used a guessable or weak password that was successfully cracked during three rapid attempts from 10.0.0.52. The temporal pattern (3 failures in 13 s, success ~3 min later, same source) is consistent with a dictionary or targeted-wordlist SSH brute-force rather than a random password.
- The 'spike' account was configured with sudo privileges (evidenced by the executed `sudo /usr/bin/id`), making it an attractive target for credential-attacking tools that prioritize users with elevated capabilities. The `id` invocation is a standard first-step privilege/groupId reconnaissance.
- Host-based SSH password authentication was enabled and no per-IP rate-limiting or lockout (e.g., fail2ban, pam_faillock) was enforced, allowing the attacker to iterate quickly across multiple password guesses from 10.0.0.52 without triggering an account lockout between attempts.
- The account 'spike' may be a shared, service, or test account whose password was known to or guessable by the operator at 10.0.0.52, rather than a result of a true online brute-force. A legitimate insider with prior knowledge of the credential could replicate the same observable pattern.

### Investigations
- Review the full audit trail or shell history for user 'spike' (e.g., /var/log/auth.log, ~/.bash_history, /var/log/secure) to identify whether commands beyond `sudo id` were executed in the session, indicating further reconnaissance or lateral movement in progress. (read-only file read / journalctl -u sshd --since …)
- Inspect the sudoers configuration (/etc/sudoers, /etc/sudoers.d/) to determine the full scope of privileges granted to 'spike'. This clarifies the blast radius of the compromise and whether the account has NOPASSWD or wildcard rules. (read-only file read)
- Check whether the 'spike' account has any SSH authorized_keys entries, recently added crontab entries, or modifications to /etc/shadow, /etc/passwd that could indicate persistence mechanisms were installed. (read-only file read)
- Cross-reference 10.0.0.52 against the threat-intelligence or internal asset inventory to determine whether this is an internal host (possible insider or compromised workstation) or an external address. (read-only lookup / DNS reverse lookup)
- Confirm from the w / last output (already partially obtained) whether the 'spike' session on pts/0 from 10.0.0.52 is still an active, connected TTY or has been disconnected. An active TTY implies the session is live and the attacker still has a foothold. (read-only: who, w, ps -t pts/0)

### Recommendations
- Immediately terminate the active 'spike' SSH session (e.g., `pkill -u spike` or `kill` the corresponding pts/0 process) to cut the attacker's live foothold, and then lock the account with `passwd -l spike` pending a full forensic review. This is an urgent containment step, not a diagnostic action.
- Block 10.0.0.52 at the host firewall (pf) or network perimeter until its origin is verified, to prevent reconnection or additional connection attempts while the investigation proceeds.
- Rotate all credentials associated with the 'spike' account and audit whether the same or similar password was reused on other systems. If 'spike' accessed other services with that password, treat the credential as globally compromised.
- Transition the host to SSH key-based authentication only (disable `PasswordAuthentication yes` in sshd_config) and enforce MFA where the business case allows. Remove SSH password authentication as a standing control to eliminate this attack vector.
- Deploy a connection-rate-limiting mechanism (pam_faillock, fail2ban, or nftables/pf rate-limit rules) to automatically throttle or block source IPs that exceed a small threshold of failed SSH password attempts within a short window, reducing the effectiveness of future brute-force campaigns.
- Perform a focused forensic pass on the host: snapshot /etc/shadow, /etc/passwd, /var/log/auth.log, all user home directories, crontab entries, .ssh/ directories, and recent file-system modifications (find / -mtime -1 -type f) to rule out persistence, privilege-escalation tooling, or data staging that may have occurred outside the single logged `id` command.
- Review the sudoers policy for 'spike' and apply the principle of least privilege. If the account is no longer needed (e.g., a test or build account), deprovision it entirely and remove it from /etc/passwd.
