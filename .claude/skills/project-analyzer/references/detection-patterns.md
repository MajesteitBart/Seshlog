# Detection Patterns Reference

Comprehensive patterns for identifying project types, frameworks, and tooling.

## Table of Contents

- [Language Detection](#language-detection)
- [Framework Detection](#framework-detection)
- [Testing Frameworks](#testing-frameworks)
- [Build Tools](#build-tools)
- [Linting & Formatting](#linting--formatting)
- [CI/CD Systems](#cicd-systems)
- [Infrastructure](#infrastructure)
- [Project Styles](#project-styles)

## Language Detection

### Node.js / JavaScript / TypeScript

| File | Indicates | Additional Info |
|------|-----------|-----------------|
| `package.json` | Node.js project | Check `type: "module"` for ESM |
| `tsconfig.json` | TypeScript | Check `target` for ES version |
| `jsconfig.json` | JavaScript with intellisense | Often in VS Code projects |
| `package-lock.json` | npm package manager | |
| `yarn.lock` | Yarn package manager | |
| `pnpm-lock.yaml` | pnpm package manager | |
| `bun.lockb` | Bun runtime | |

### Python

| File | Indicates | Additional Info |
|------|-----------|-----------------|
| `pyproject.toml` | Modern Python project | Check `[build-system]` for tooling |
| `requirements.txt` | pip dependencies | Legacy but common |
| `setup.py` | Setuptools package | Often legacy |
| `poetry.lock` | Poetry package manager | |
| `Pipfile` | Pipenv | |
| `uv.lock` | uv package manager | Modern, fast |

### Rust

| File | Indicates | Additional Info |
|------|-----------|-----------------|
| `Cargo.toml` | Rust project | Check `[workspace]` for monorepo |
| `Cargo.lock` | Locked dependencies | |
| `rust-toolchain.toml` | Rust version pinning | |

### Go

| File | Indicates | Additional Info |
|------|-----------|-----------------|
| `go.mod` | Go module | Check Go version |
| `go.sum` | Locked dependencies | |
| `go.work` | Go workspace (monorepo) | |

### Java / JVM

| File | Indicates | Additional Info |
|------|-----------|-----------------|
| `pom.xml` | Maven project | |
| `build.gradle` / `build.gradle.kts` | Gradle project | `.kts` = Kotlin DSL |
| `settings.gradle` | Multi-module Gradle | |
| `.mvn/` | Maven wrapper | |
| `gradlew` | Gradle wrapper | |

### Other Languages

| File | Language | Additional Info |
|------|----------|-----------------|
| `Gemfile` | Ruby | Check for Rails indicators |
| `composer.json` | PHP | Check for Laravel/Symfony |
| `mix.exs` | Elixir | Phoenix framework common |
| `pubspec.yaml` | Dart/Flutter | |
| `*.csproj` | C# / .NET | |
| `CMakeLists.txt` | C/C++ | |
| `Makefile` | Various | Often C/C++/Go |

## Framework Detection

### JavaScript/TypeScript Frameworks

| Pattern | Framework | Notes |
|---------|-----------|-------|
| `next.config.js/ts/mjs` | Next.js | Check for App Router vs Pages |
| `nuxt.config.ts` | Nuxt | Vue-based |
| `vite.config.ts` | Vite | Often with React/Vue/Svelte |
| `angular.json` | Angular | Enterprise SPA |
| `svelte.config.js` | SvelteKit | |
| `astro.config.mjs` | Astro | Content-focused |
| `remix.config.js` | Remix | Full-stack React |
| `gatsby-config.js` | Gatsby | Static site generator |
| `electron-builder.json` | Electron | Desktop apps |

### Python Frameworks

| Pattern | Framework | Notes |
|---------|-----------|-------|
| `manage.py` + `settings.py` | Django | Check for DRF |
| `from fastapi import FastAPI` | FastAPI | Modern async API |
| `from flask import Flask` | Flask | Lightweight |
| `streamlit` in deps | Streamlit | Data apps |
| `from django.contrib.admin` | Django Admin | |

### Backend/API Indicators

| Pattern | Indicates |
|---------|-----------|
| `express` in deps | Express.js API |
| `@nestjs/core` in deps | NestJS |
| `hono` in deps | Hono (edge-first) |
| `fastify` in deps | Fastify |
| `koa` in deps | Koa |

## Testing Frameworks

### JavaScript/TypeScript

| Config File | Framework | Notes |
|-------------|-----------|-------|
| `jest.config.*` | Jest | Most common |
| `vitest.config.*` | Vitest | Vite-native, faster |
| `playwright.config.*` | Playwright | E2E testing |
| `cypress.config.*` | Cypress | E2E testing |
| `.mocharc.*` | Mocha | Often with Chai |
| `karma.conf.js` | Karma | Angular legacy |

### Python

| Config | Framework | Notes |
|--------|-----------|-------|
| `pytest.ini` | Pytest | Most common |
| `[tool.pytest]` in pyproject.toml | Pytest | Modern config |
| `tox.ini` | Tox | Multi-env testing |
| `noxfile.py` | Nox | Modern tox alternative |

### Other

| Pattern | Framework | Language |
|---------|-----------|----------|
| `spec/` directory | RSpec | Ruby |
| `*_test.go` | Go testing | Go |
| `tests/` + Cargo.toml | Rust testing | Rust |

## Build Tools

| File | Tool | Notes |
|------|------|-------|
| `webpack.config.js` | Webpack | Legacy bundler |
| `rollup.config.js` | Rollup | Library bundler |
| `esbuild.config.*` | esbuild | Fast bundler |
| `tsup.config.ts` | tsup | TS library bundler |
| `turbo.json` | Turborepo | Monorepo build |
| `nx.json` | Nx | Monorepo tooling |
| `lerna.json` | Lerna | Legacy monorepo |

## Linting & Formatting

### JavaScript/TypeScript

| File | Tool |
|------|------|
| `.eslintrc*` / `eslint.config.*` | ESLint |
| `.prettierrc*` | Prettier |
| `biome.json` | Biome (ESLint + Prettier) |
| `.stylelintrc*` | Stylelint (CSS) |

### Python

| File | Tool |
|------|------|
| `ruff.toml` / `[tool.ruff]` | Ruff (fast linter) |
| `.flake8` | Flake8 |
| `pyproject.toml[tool.black]` | Black formatter |
| `.isort.cfg` | isort (imports) |
| `.mypy.ini` / `[tool.mypy]` | mypy (types) |
| `pyrightconfig.json` | Pyright (types) |

## CI/CD Systems

| Path/File | System |
|-----------|--------|
| `.github/workflows/` | GitHub Actions |
| `.gitlab-ci.yml` | GitLab CI |
| `Jenkinsfile` | Jenkins |
| `.circleci/config.yml` | CircleCI |
| `.travis.yml` | Travis CI |
| `azure-pipelines.yml` | Azure DevOps |
| `bitbucket-pipelines.yml` | Bitbucket |
| `.drone.yml` | Drone CI |

## Infrastructure

### Containerization

| File | Indicates |
|------|-----------|
| `Dockerfile` | Docker container |
| `docker-compose.yml` | Multi-container setup |
| `compose.yaml` | Docker Compose v2 |
| `.dockerignore` | Docker build context |

### Deployment

| File | Platform |
|------|----------|
| `vercel.json` | Vercel |
| `netlify.toml` | Netlify |
| `fly.toml` | Fly.io |
| `render.yaml` | Render |
| `railway.json` | Railway |
| `app.yaml` | Google App Engine |
| `serverless.yml` | Serverless Framework |
| `sam.yaml` / `template.yaml` | AWS SAM |

### Infrastructure as Code

| File | Tool |
|------|------|
| `*.tf` | Terraform |
| `pulumi.*` | Pulumi |
| `cdk.json` | AWS CDK |
| `cloudformation.yaml` | CloudFormation |

## Project Styles

### Monorepo Indicators

| Pattern | Tool/Style |
|---------|------------|
| `turbo.json` | Turborepo monorepo |
| `nx.json` | Nx monorepo |
| `lerna.json` | Lerna monorepo |
| `pnpm-workspace.yaml` | pnpm workspaces |
| `packages/` + root package.json | npm/yarn workspaces |
| `apps/` + `packages/` | Typical monorepo structure |
| `[workspace]` in Cargo.toml | Rust workspace |
| `go.work` | Go workspace |

### Library Indicators

| Pattern | Indicates |
|---------|-----------|
| `exports` in package.json | npm library |
| `[lib]` in Cargo.toml | Rust library |
| `[tool.poetry]` with classifiers | Python package |
| `main` + `types` in package.json | TypeScript library |

### CLI Indicators

| Pattern | Indicates |
|---------|-----------|
| `bin` in package.json | Node.js CLI |
| `[[bin]]` in Cargo.toml | Rust CLI |
| `[project.scripts]` in pyproject.toml | Python CLI |
| `commander`/`yargs`/`clipanion` in deps | Node.js CLI framework |
| `click`/`typer`/`argparse` in deps | Python CLI framework |
| `clap` in Cargo.toml deps | Rust CLI framework |
