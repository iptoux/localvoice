# Keep npm and pnpm lockfiles aligned for security audits

**Date:** 2026-04-25
**Area:** Frontend / CI / Dependency Management
**Milestone:** Release pipeline

## What Happened

After `pnpm update --latest`, `pnpm audit` reported no known vulnerabilities, but `npm audit` still reported advisories against `package-lock.json`.

## Root Cause

The repository tracks both `pnpm-lock.yaml` and `package-lock.json`. Updating dependencies with pnpm only updates the pnpm lockfile. GitHub security scanning can still read the npm lockfile, so stale vulnerable transitive versions there continue to appear as security alerts.

## Fix / Solution

Ran `npm install --package-lock-only --ignore-scripts` to refresh `package-lock.json`, then `npm audit fix --package-lock-only --ignore-scripts` to update vulnerable npm lockfile transitive entries without changing installed modules or running package scripts.

## Learning / Rule of Thumb

When a repository tracks multiple JavaScript lockfiles, audit and refresh all tracked lockfiles before considering a security update complete.

## References

- `package-lock.json`
- `pnpm-lock.yaml`
