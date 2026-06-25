/**
 * Canvas 渲染器
 * 负责特效的Canvas动画渲染
 */

import { CanvasRenderer, RenderParams, AnimationType } from './types';

/**
 * 动画帧接口
 */
interface AnimationFrame {
  id: string;
  params: RenderParams;
  startTime: number;
  animationId: number;
}

/**
 * Canvas渲染器实现
 */
export class CanvasRendererImpl implements CanvasRenderer {
  private canvas: HTMLCanvasElement;
  private ctx: CanvasRenderingContext2D;
  private animationFrames: Map<string, AnimationFrame> = new Map();
  private animationId: number = 0;
  private isRunning: boolean = false;

  constructor(canvas: HTMLCanvasElement) {
    this.canvas = canvas;
    const ctx = canvas.getContext('2d');

    if (!ctx) {
      throw new Error('无法获取Canvas 2D上下文');
    }

    this.ctx = ctx;

    // 设置Canvas尺寸（使用父容器尺寸或默认值）
    const parent = canvas.parentElement;
    const width = parent ? parent.clientWidth : 800;
    const height = parent ? parent.clientHeight : 600;
    this.resize(width, height);

    // 监听窗口大小变化
    window.addEventListener('resize', () => {
      const parent = canvas.parentElement;
      const width = parent ? parent.clientWidth : 800;
      const height = parent ? parent.clientHeight : 600;
      this.resize(width, height);
    });
  }

  /**
   * 调整Canvas尺寸
   * @param width 宽度
   * @param height 高度
   */
  resize(width: number, height: number): void {
    this.canvas.width = width;
    this.canvas.height = height;
  }

  /**
   * 渲染特效
   * @param params 渲染参数
   */
  async render(params: RenderParams): Promise<void> {
    const id = `effect_${this.animationId++}`;

    // 创建动画帧
    const frame: AnimationFrame = {
      id,
      params,
      startTime: performance.now(),
      animationId: requestAnimationFrame(() => this.animationLoop())
    };

    this.animationFrames.set(id, frame);

    // 如果动画循环未启动，则启动
    if (!this.isRunning) {
      this.isRunning = true;
      this.animationLoop();
    }

    return Promise.resolve();
  }

  /**
   * 动画循环
   */
  private animationLoop(): void {
    const currentTime = performance.now();

    // 清除画布
    this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);

    // 渲染所有活动动画
    const activeFrames: AnimationFrame[] = [];

    for (const [_id, frame] of this.animationFrames) {
      const elapsed = currentTime - frame.startTime;

      // 检查动画是否过期
      if (elapsed < frame.params.duration) {
        this.renderFrame(frame, elapsed);
        activeFrames.push(frame);
      } else {
        // 移除过期动画
        cancelAnimationFrame(frame.animationId);
      }
    }

    this.animationFrames = new Map(activeFrames.map(frame => [frame.id, frame]));

    // 如果还有活动动画，继续循环
    if (this.animationFrames.size > 0) {
      requestAnimationFrame(() => this.animationLoop());
    } else {
      this.isRunning = false;
    }
  }

  /**
   * 渲染单个动画帧
   * @param frame 动画帧
   * @param elapsed 已经过去的时间
   */
  private renderFrame(frame: AnimationFrame, elapsed: number): void {
    const { params } = frame;
    const progress = Math.min(elapsed / params.duration, 1);

    // 计算动画参数
    const { x, y, fontSize, primaryColor, secondaryColor, animationType } = params;

    // 根据动画类型计算位置和透明度
    const posX = x;
    let posY = y;
    let opacity = 1;
    let scale = 1;
    let rotation = 0;

    switch (animationType) {
      case AnimationType.FADE: {
        opacity = 1 - progress;
        break;
      }

      case AnimationType.SLIDE: {
        posY = y + (1 - progress) * 100;
        opacity = 1 - progress;
        break;
      }

      case AnimationType.SCALE: {
        scale = 1 + progress * 0.5;
        opacity = 1 - progress;
        break;
      }

      case AnimationType.ROTATE: {
        rotation = progress * Math.PI * 2;
        opacity = 1 - progress;
        break;
      }

      case AnimationType.PARTICLE: {
        // 简单的粒子效果
        const particleCount = 20;
        for (let i = 0; i < particleCount; i++) {
          const angle = (i / particleCount) * Math.PI * 2;
          const distance = progress * 100;
          const particleX = x + Math.cos(angle) * distance;
          const particleY = y + Math.sin(angle) * distance;
          const particleOpacity = 1 - progress;

          this.ctx.globalAlpha = particleOpacity;
          this.ctx.fillStyle = primaryColor;
          this.ctx.beginPath();
          this.ctx.arc(particleX, particleY, 2, 0, Math.PI * 2);
          this.ctx.fill();
        }
        return; // 粒子效果不需要渲染文本
      }

      default:
        break;
    }

    // 设置渲染状态
    this.ctx.save();
    this.ctx.globalAlpha = opacity;
    this.ctx.translate(posX, posY);
    this.ctx.scale(scale, scale);
    this.ctx.rotate(rotation);

    // 创建渐变
    const gradient = this.ctx.createLinearGradient(-100, 0, 100, 0);
    gradient.addColorStop(0, primaryColor);
    gradient.addColorStop(1, secondaryColor);

    // 设置字体和样式
    this.ctx.font = `bold ${fontSize}px Arial, sans-serif`;
    this.ctx.fillStyle = gradient;
    this.ctx.textAlign = 'center';
    this.ctx.textBaseline = 'middle';

    // 添加阴影效果
    this.ctx.shadowColor = 'rgba(0, 0, 0, 0.5)';
    this.ctx.shadowBlur = 4;
    this.ctx.shadowOffsetX = 2;
    this.ctx.shadowOffsetY = 2;

    // 渲染文本
    this.ctx.fillText(params.text, 0, 0);

    this.ctx.restore();
  }

  /**
   * 清除所有动画
   */
  clear(): void {
    // 取消所有动画帧
    for (const frame of this.animationFrames.values()) {
      cancelAnimationFrame(frame.animationId);
    }

    this.animationFrames.clear();
    this.isRunning = false;

    // 清除画布
    this.ctx.clearRect(0, 0, this.canvas.width, this.canvas.height);
  }

  /**
   * 获取活动动画数量
   * @returns 活动动画数量
   */
  getActiveAnimationCount(): number {
    return this.animationFrames.size;
  }

  /**
   * 检查是否有活动动画
   * @returns 是否有活动动画
   */
  hasActiveAnimations(): boolean {
    return this.animationFrames.size > 0;
  }
}

/**
 * 创建Canvas渲染器工厂函数
 * @param canvas Canvas元素
 * @returns Canvas渲染器实例
 */
export function createCanvasRenderer(canvas: HTMLCanvasElement): CanvasRendererImpl {
  return new CanvasRendererImpl(canvas);
}
