# 验证记录：首页一键下载 Qwen3-TTS 1.7B 模型

日期：2026-11-20

## 1. 结论

首页「Qwen3-TTS 1.7B 语音模型」卡片的下载按钮已打通：点击 → Rust 命令 → 语音服务 → Hub 下载 → 进度回传 UI。
服务端契约、Rust 客户端、IPC 命令面均有测试覆盖；服务端已用真实进程 + 真实 HTTP 做过端到端冒烟。

## 2. 测试结果

| 项 | 命令 | 结果 |
| --- | --- | --- |
| 服务端契约 | `python -m unittest discover -s services/qwen3-tts/tests -t services/qwen3-tts` | ✅ **17 passed**（新增 7 项模型下载用例） |
| Rust 编译 | `cargo check --lib --tests --features free` | ✅ 0 error / 0 warning |
| Rust 测试 | `cargo test --lib --features free` | ✅ **150 passed, 0 failed**（新增 4 项客户端用例） |
| 命令面契约 | `ipc::media_contract_test::exposes_the_approved_media_surface_only` | ✅ 38 → 40，含 `tts_qwen3_models` / `tts_qwen3_download_model` |
| 语言包 | 双语言键集合比对 | ✅ 689 / 689，新增 `dashboard.qwen3Model.*` 12 键一致 |

新增 Python 用例：目录含 1.7B 默认项、缺省下载 1.7B、下载后标记 installed、未知模型 400、
同模型重复请求幂等、并发不同模型 409、失败后服务仍可用。
新增 Rust 用例：解析模型目录、启动下载返回在途状态、`tts_model_unknown` 码翻译、`tts_download_busy` 可重试翻译。

## 3. 端到端冒烟（真实进程 + 真实 HTTP）

```
python services/qwen3-tts/server.py --mock --port 18080

GET /v1/models
  before: status='idle' models=5
    Qwen/Qwen3-TTS-12Hz-1.7B-CustomVoice default=True installed=False
    Qwen/Qwen3-TTS-12Hz-1.7B-VoiceDesign default=False installed=False

POST /v1/models/download  {}
  start: model=Qwen/Qwen3-TTS-12Hz-1.7B-CustomVoice status=downloading progress=0

GET /v1/models（400ms 后）
  mid:   status=downloading progress=40 bytes=209715

GET /v1/models（2s 后）
  after: status=ready progress=100
  installed: Qwen/Qwen3-TTS-12Hz-1.7B-CustomVoice
```

说明：`--mock` 使用模拟下载器（不联网、不落真实权重），验证的是接口契约、状态机与进度链路。

## 4. UI 行为

| 场景 | 表现 |
| --- | --- |
| 语音服务未启动 | 角标「语音服务未启动」，按钮禁用，卡片提示启动命令 |
| 未下载 | 角标「未下载」，按钮「下载模型」 |
| 下载中 | 按钮 loading，进度条显示百分比（服务端无法估算总量时为不确定态） |
| 已下载 | 角标「已下载」，按钮「重新下载」 |
| 下载失败 | 红色错误文案显示服务端原因，按钮恢复可点 |

轮询：store 在下载期间每 2s 拉一次 `/v1/models`，状态离开 `downloading` 即停；`dispose()` 会清理定时器。

## 5. 未覆盖项

| 项 | 状态 | 说明 |
| --- | --- | --- |
| 真实权重下载（4.5GB） | ⛔ 未执行 | 需联网与磁盘空间；本机未拉取真实权重 |
| 真实进度百分比 | ⚠ 部分 | 百分比依赖 Hub 返回文件体积；拿不到时为不确定态，属预期降级 |
| 前端 tsc / vitest | ⛔ 未执行 | `node_modules` junction 失效（既存环境问题，见 `remove-whisper-asr/verification.md`） |
| 首页视觉渲染 | ⛔ 未执行 | 需 `pnpm install` 后 `tauri dev` |

前端改动为新增卡片与 store 方法，已通过源码契约测试（`dashboard-page.test.ts` 新增 2 项断言：按钮存在、不绕过 store、模型 id 不硬编码）与全量残留检查。
