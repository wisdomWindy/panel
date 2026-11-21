# Qwen3-TTS 语音服务

为 `biliLiveHelper` 提供 Qwen3-TTS 语音合成，供两条链路共用：

- 语音播报（直播间事件模板）
- AI 语音回复（LLM 回复朗读）

上游参考：[QwenLM/Qwen3-TTS](https://github.com/QwenLM/Qwen3-TTS#model-architecture)（Apache-2.0）

## 1. 为什么是独立进程

模型运行在 Python + PyTorch 上，无法内联进 Rust 宿主；上游也明确建议使用独立环境。
因此本服务独立部署，桌面端只通过 HTTP 调用，未启动时客户端自动回退系统语音。

## 2. 快速开始

### 2.1 无 GPU 的契约自检（推荐先做）

```bash
python services/qwen3-tts/server.py --mock
```

`--mock` 用合成音代替模型，**只验证接口契约与链路连通**，不产生真实语音。

自检：

```bash
python -m unittest discover -s services/qwen3-tts/tests -t services/qwen3-tts -v
python services/qwen3-tts/bench.py            # 传输层延迟基线
```

### 2.2 真实模型

```bash
conda create -n qwen3-tts python=3.12 -y
conda activate qwen3-tts
pip install -r services/qwen3-tts/requirements.txt

# 国内网络建议先用 ModelScope 下载权重（见上游 README）
python services/qwen3-tts/server.py \
  --model Qwen/Qwen3-TTS-12Hz-1.7B-CustomVoice \
  --device cuda:0 --dtype bfloat16 --attn flash_attention_2
```

首次启动会下载权重（1.7B 约 4.5GB），加载期间 `/health` 返回 `"status":"loading"`，
合成请求返回 `503 tts_model_loading`（可重试）。

低显存机器改用 0.6B：

```bash
python services/qwen3-tts/server.py --model Qwen/Qwen3-TTS-12Hz-0.6B-CustomVoice
```

声音设计模型（用自然语言描述音色）：

```bash
python services/qwen3-tts/server.py \
  --model Qwen/Qwen3-TTS-12Hz-1.7B-VoiceDesign --task voice_design
```

此时 `POST /v1/audio/speech` 必须携带 `instruct`，否则返回 `400 tts_instruct_required`。

## 3. 接口

| 方法 | 路径 | 说明 |
| --- | --- | --- |
| GET | `/health` | 就绪状态、模型名、实际采样率、音色清单 |
| GET | `/v1/voices` | 音色目录 |
| GET | `/v1/models` | 已发布模型清单 + 当前下载状态 |
| POST | `/v1/models/download` | 下载指定模型（缺省为 1.7B CustomVoice） |
| POST | `/v1/audio/speech` | 合成，返回 `audio/wav` |

```bash
curl -s http://127.0.0.1:8020/health
curl -s -X POST http://127.0.0.1:8020/v1/audio/speech \
  -H "Content-Type: application/json" \
  -d '{"text":"用户A说：今天直播真不错","voice":"Serena","language":"Chinese"}' \
  --output out.wav

# 下载 1.7B 模型（应用首页的「下载模型」按钮走的就是这两个接口）
curl -s http://127.0.0.1:8020/v1/models
curl -s -X POST http://127.0.0.1:8020/v1/models/download \
  -H "Content-Type: application/json" -d '{}'
```

### 模型下载

- 下载在服务端后台线程执行，接口立即返回 `202` 与快照；进度通过反复 `GET /v1/models` 的 `download` 字段读取。
- 权重落到 Hugging Face 缓存（或 `--cache-dir`），随后 `from_pretrained` 直接命中，无需重复下载。
- 优先用 `huggingface_hub`，缺失时回退 `modelscope`（国内镜像）；两者都没有则返回 `503 tts_download_unavailable`。
- 同一模型重复请求是幂等的；并发下载不同模型返回 `409 tts_download_busy`。
- 未知模型返回 `400 tts_model_unknown`。
- `--mock` 模式使用模拟下载器，落一个 marker 文件，便于无网络演示整条 UI 流程。

请求字段：`text`（必填，≤2000 字）、`voice`、`language`（11 个合法值）、
`instruct`（自然语言语气/语速）、`response_format`（仅 `wav`）。

错误统一为 `{"error":{"code","message","retryable"}}`。

## 4. 桌面端配置

首页（数据看板）底部有「Qwen3-TTS 1.7B 语音模型」卡片：

- 语音服务未启动时显示「语音服务未启动」并禁用按钮；
- 点击「下载模型」调用本服务 `POST /v1/models/download`，卡片上实时显示进度；
- 下载完成后角标变为「已下载」，按钮变为「重新下载」。

其余参数在「语音播报 → Qwen3-TTS 语音服务」区域，或直接写设置：

| 项 | 默认 | 说明 |
| --- | --- | --- |
| 启用 | 关 | 关闭后不发起任何合成请求 |
| 服务地址 | `http://127.0.0.1:8020` | 可指向局域网/远程；需 http(s) |
| 语言 | `Chinese` | `Auto` 由模型自适应 |
| 播报音色 | `Serena` | 也可在前端“音色”下拉里直接选 |
| 播报指令 | 空 | 如“温柔一点，语速稍慢” |
| 超时 | 8000ms | 500–60000 |
| 失败回退系统语音 | 开 | 关闭后失败即静音 |
| AI 回复音色 | 空 | 空则复用播报音色 |
| AI 回复指令 | 空 | 空则复用播报指令 |

AI 语音回复的可用音色在 AI 页面以 `qwen3:` 前缀出现；选择系统音色即为该条回复回滚。

## 5. 运维

- **健康检查**：`GET /health`，`ready=true` 且 `status=ok` 才算可用。
- **日志**：服务端把访问日志写 stderr；应用端错误码见应用日志与 `qwen3_tts_*`。
- **端口**：默认 `127.0.0.1:8020`，仅监听本机；如需远程访问请自行加反向代理与鉴权（本服务无鉴权，**不要直接暴露公网**）。
- **并发**：单进程内推理串行（全局锁），不要多开同卡实例。
- **显存**：OOM 时降级到 0.6B 或 `--dtype float16`；`--attn ''` 可关闭 FlashAttention。
- **停止服务**：桌面端会在下一次合成失败后自动回退系统语音，无需重启应用。

## 6. Windows 开机自启（可选）

```powershell
# 以当前用户创建计划任务，登录后启动
$action  = New-ScheduledTaskAction -Execute "pythonw.exe" -Argument "D:\path\to\services\qwen3-tts\server.py"
$trigger = New-ScheduledTaskTrigger -AtLogOn
Register-ScheduledTask -TaskName "Qwen3-TTS Speech Service" -Action $action -Trigger $trigger
```

## 7. 已知边界

- 未实现流式首包播放（一期整段合成）。上游支持流式，可作为二期优化。
- 未实现音频缓存：播报文本含用户名，命中率低。
- `rate`（语速倍率）折算为指令文本，不做重采样，因此实际语速由模型理解决定。
