# mdbook

`mdbook` 是一款用于创建书籍的工具，书籍的内容是使用 Markdown 语法编写的。

## 安装

```bash
cargo install mdbook

# 自动补全
mkdir ~/.zfunc
# 在 .zshrc 文件中添加以下内容：
# fpath+=~/.zfunc
# autoload -Uz compinit && compinit

mdbook completions zsh > ~/.zfunc/_mdbook
exec zsh
```

## 创建项目

```bash
mkdir learning-rust && cd learning-rust
mdbook init . --theme
```

## 启动服务

```bash
mdbook serve --open
```

## 配置 book.toml

```bash
vim "./book.toml"
```

编辑 `book.toml` 文件，添加或修改配置：

```toml
[book]
authors = ["poneding"]
language = "zh-CN"
multilingual = false
src = "notes"
title = "Learning Rust"

[build]
build-dir = "notes/dist"

[output.html]
cname = "learning-rust.poneding.com"
default-theme = "light"
preferred-dark-theme = "navy"
git-repository-url = "https://github.com/poneding/learning-rust"
git-repository-icon = "fa-github"
edit-url-template = "https://github.com/poneding/learning-rust/edit/master/{path}"
```

## 集成 Giscus 评论系统

1、参考说明：<https://giscus.app/zh-CN>，获取到 js 代码，格式如下：

```html
<script src="https://giscus.app/client.js"
        data-repo="[在此输入仓库]"
        data-repo-id="[在此输入仓库 ID]"
        data-category="[在此输入分类名]"
        data-category-id="[在此输入分类 ID]"
        data-mapping="pathname"
        data-strict="0"
        data-reactions-enabled="1"
        data-emit-metadata="0"
        data-input-position="bottom"
        data-theme="preferred_color_scheme"
        data-lang="zh-CN"
        crossorigin="anonymous"
        async>
</script>
```

2、修改主题文件

```bash
vim theme/index.hbs
```

在 `{{{ content }}}` 下方添加获取到的代码，最终内容如下：

```html
                    <main>
                        {{{ content }}}
                        <script src="https://giscus.app/client.js"
                                data-repo="poneding/learning-rust"
                                data-repo-id="R_kgDOHvlGbg"
                                data-category="General"
                                data-category-id="DIC_kwDOHvlGbs4ChIg2"
                                data-mapping="pathname"
                                data-strict="0"
                                data-reactions-enabled="1"
                                data-emit-metadata="0"
                                data-input-position="top"
                                data-theme="preferred_color_scheme"
                                data-lang="zh-CN"
                                data-loading="lazy"
                                crossorigin="anonymous"
                                async>
                        </script>
                    </main>
```

## GitHub Pages 部署

创建 `.github/workflows/deploy.yml` 文件，写入如下内容：

```yml
name: Deploy
on:
  push:
    branches:
      - master

jobs:
  deploy:
    runs-on: ubuntu-latest
    permissions:
      contents: write  # To push a branch 
      pull-requests: write  # To create a PR from that branch
    steps:
    - uses: actions/checkout@v4
      with:
        fetch-depth: 0
    - name: Install latest mdbook
      run: |
        tag=$(curl 'https://api.github.com/repos/rust-lang/mdbook/releases/latest' | jq -r '.tag_name')
        url="https://github.com/rust-lang/mdbook/releases/download/${tag}/mdbook-${tag}-x86_64-unknown-linux-gnu.tar.gz"
        mkdir mdbook
        curl -sSL $url | tar -xz --directory=./mdbook
        echo `pwd`/mdbook >> $GITHUB_PATH
    - name: Deploy GitHub Pages
      run: |
        # This assumes your book is in the root of your repository.
        # Just add a `cd` here if you need to change to another directory.
        mdbook build
        git worktree add gh-pages
        git config user.name "Deploy from CI"
        git config user.email ""
        cd gh-pages
        # Delete the ref to avoid keeping history.
        git update-ref -d refs/heads/gh-pages
        rm -rf *
        mv ../notes/dist/* .
        git add .
        git commit -m "Deploy $GITHUB_SHA to gh-pages"
        git push --force --set-upstream origin gh-pages
```

> `mv ../notes/dist/* .` 是自定义配置打包路径，来自 `book.toml` build-dir 配置项，默认配置是 `mv ../book/* .`
>
> .gitignore 添加 `notes/dist` 配置，避免提交 dist 目录
