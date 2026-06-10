## Contributing

Contributions are very welcome. To keep things clean and mergeable, please follow these conventions.

### Branch naming

All work branches must follow this pattern:

```
<type>/<short-description>
```

| Type | When to use |
|------|-------------|
| `feat/` | New feature or capability |
| `fix/` | Bug fix |
| `theme/` | Theming-related work |
| `docs/` | Documentation only |
| `refactor/` | Code restructure, no behavior change |
| `perf/` | Performance improvement |
| `chore/` | Tooling, CI, dependencies |
| `test/` | Tests only |

**Examples:**
```
feat/theme-engine
fix/aur-cache-fallback
docs/contributing-guide
refactor/event-loop-cleanup
theme/catppuccin-mocha
```

> ⚠ PRs from branches that don't match this pattern will be asked to rename before review.

### Workflow

```bash
# 1. Fork the repo and clone your fork
git clone https://github.com/pathakjiop/pkgman.git
cd pkgman

# 2. Create your branch from main
git checkout -b feat/your-feature-name

# 3. Make your changes, commit with a clear message
git commit -m "feat: add catppuccin theme support"

# 4. Push and open a PR against main
git push origin feat/your-feature-name
```

### Commit message format

Follow the conventional commit style:

```
<type>: <short imperative description>

Optional longer body explaining the why, not the what.
```

### PR checklist

Before opening a PR, make sure:

- [ ] `cargo build --release` succeeds with no warnings
- [ ] Your branch name follows the naming convention above
- [ ] Changes are scoped — one concern per PR
- [ ] You've updated documentation if behavior changed
- [ ] For new features, a brief description is in the PR body

### Good first issues

Look for issues tagged [`good first issue`](https://github.com/pathakjiop/pkgman/issues?q=label%3A"good+first+issue") — these are intentionally scoped for newcomers.

---

## Roadmap

- [ ] **Theming engine** — full color/style customization via `config.toml`
- [ ] **Built-in theme presets** — Catppuccin, Nord, Gruvbox, Tokyo Night
- [ ] **Mouse support** — optional click-to-select
- [ ] **AUR comment viewer** — inline AUR comments and flag status
- [ ] **Dependency tree visualizer** — graphical dep tree in the details pane
- [ ] **AUR PKGBUILD diff viewer** — diff updates before installing
