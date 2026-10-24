# Bugfix Source

## Verbatim Evidence

- U1: "红框中的字颜色太浅看不清楚"
- U2: "系统设置-界面中的主题默认是跟随系统"
- U3: "配置管理页面的配置位置默认是软件安装的目录"

## Observed Code Facts

- Dashboard metric cards use `bg-white`; their `<dd>` values have no text color and inherit dark-shell light text.
- Renderer and Rust UI defaults both use `dark`.
- Startup builds `app.db` under Tauri `app_data_dir`; config overview displays its parent.

## Closed-World Scope

Only metric value contrast, the UI theme default, release database location, and first-run legacy database preservation may change. No layout, copy, API shape, export path, recording path, or unrelated page theme work is authorized.
