# Records assets/demo.gif. Run from the repo root: `vhs assets/demo.tape`.
#
# The wizard runs against a local copy of this repo placed at
# ./napalmpapalam/rust-template: the `owner/repo` shorthand resolves a local
# path before it tries GitHub, so the command on screen is the real one. Drop
# the two hidden setup lines once the repo is public and it records against the
# real remote — the "not found in config" warning goes with them.

Output assets/demo.gif

Set Shell bash
Set FontSize 16
Set Width 1180
Set Height 900
Set Padding 24
Set Theme "Catppuccin Mocha"
Set TypingSpeed 40ms

Hide
Type `export PS1="\[\e[38;5;175m\]❯\[\e[0m\] " CLICOLOR=1`
Enter
Type `rm -rf /tmp/rt-demo && mkdir -p /tmp/rt-demo/napalmpapalam && cp -R "$PWD" /tmp/rt-demo/napalmpapalam/rust-template && rm -rf /tmp/rt-demo/napalmpapalam/rust-template/.git && cd /tmp/rt-demo`
Enter
Type `clear`
Enter
Sleep 1s
Show

Type "cargo generate napalmpapalam/rust-template template"
Sleep 800ms
Enter
Sleep 2s

Type "my-service"
Sleep 500ms
Enter
Sleep 900ms

Type "Ingests events and serves them over HTTP"
Sleep 500ms
Enter
Sleep 1s

# layout: workspace
Enter
Sleep 900ms

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
