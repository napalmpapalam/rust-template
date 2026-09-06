# Records assets/demo.gif. Run from the repo root: `vhs assets/demo.tape`.
# It clones the published repo, so push before re-recording.

Output assets/demo.gif

Set Shell bash
Set FontSize 16
Set Width 1180
Set Height 880
Set Padding 24
Set Theme "Catppuccin Mocha"
Set TypingSpeed 40ms

Hide
Type `export PS1="\[\e[38;5;175m\]❯\[\e[0m\] " CLICOLOR=1`
Enter
Type `rm -rf /tmp/rt-demo && mkdir -p /tmp/rt-demo && cd /tmp/rt-demo`
Enter
Type `clear`
Enter
Sleep 1s
Show

Type "cargo generate --git https://github.com/napalmpapalam/rust-template"
Sleep 800ms
Enter
Sleep 4s

Type "my-service"
Sleep 500ms
Enter
Sleep 1200ms

Type "Ingests events and serves them over HTTP"
Sleep 500ms
Enter
Sleep 1200ms

# HTTP server: yes
Enter
Sleep 900ms

# license: MIT
Enter
Sleep 900ms

# CI: GitHub Actions
Enter
Sleep 900ms

# Dockerfile: yes
Enter
Sleep 900ms

# git hooks: yes
Enter
Sleep 900ms

# CLAUDE.md: yes
Enter
Sleep 3s

Type "cd my-service && tree -a -L 2 --dirsfirst -I .git"
Sleep 500ms
Enter
Sleep 5s
