/**
 * WebGL 3D 特效渲染器 (Three.js)
 * 从 Bilibili API 动态加载礼物图片并播放入场-展示-出场动画
 */

import { AdditiveBlending, NormalBlending } from 'three/src/constants.js';
import { BufferAttribute } from 'three/src/core/BufferAttribute.js';
import { BufferGeometry } from 'three/src/core/BufferGeometry.js';
import { PerspectiveCamera } from 'three/src/cameras/PerspectiveCamera.js';
import { TextureLoader } from 'three/src/loaders/TextureLoader.js';
import { AmbientLight } from 'three/src/lights/AmbientLight.js';
import { Light } from 'three/src/lights/Light.js';
import { PointLight } from 'three/src/lights/PointLight.js';
import { Color } from 'three/src/math/Color.js';
import { Group } from 'three/src/objects/Group.js';
import { Line } from 'three/src/objects/Line.js';
import { Mesh } from 'three/src/objects/Mesh.js';
import { Points } from 'three/src/objects/Points.js';
import { Sprite } from 'three/src/objects/Sprite.js';
import { Material } from 'three/src/materials/Material.js';
import { PointsMaterial } from 'three/src/materials/PointsMaterial.js';
import { SpriteMaterial } from 'three/src/materials/SpriteMaterial.js';
import { WebGLRenderer as ThreeWebGLRenderer } from 'three/src/renderers/WebGLRenderer.js';
import { Scene } from 'three/src/scenes/Scene.js';
import { Texture } from 'three/src/textures/Texture.js';
import { EffectConfig, EffectTriggerData, EffectType } from './types';

type AnimationPhase = 'entry' | 'display' | 'exit';

interface ActiveEffect {
  id: string;
  type: EffectType;
  group: Group;
  startTime: number;
  duration: number;
  phase: AnimationPhase;
  phaseStartTime: number;
  entryDuration: number;
  displayDuration: number;
  exitDuration: number;
  sprite: Sprite | Points;
  particles: Points | null;
  light: PointLight | null;
}

/** Bilibili 礼物图片 URL */
function getGiftImageUrl(giftId: number | string): string {
  return `https://s1.hdslb.com/bfs/live/${giftId}.png`;
}

export class WebGLRenderer {
  private scene: Scene;
  private camera: PerspectiveCamera;
  private renderer: ThreeWebGLRenderer;
  private activeEffects: Map<string, ActiveEffect> = new Map();
  private animationId: number = 0;
  private isRunning: boolean = false;
  private textureLoader: TextureLoader;

  constructor(private canvas: HTMLCanvasElement) {
    this.scene = new Scene();
    this.textureLoader = new TextureLoader();

    this.camera = new PerspectiveCamera(
      45,
      canvas.width / canvas.height,
      0.1,
      2000
    );
    this.camera.position.set(0, 0, 600);
    this.camera.lookAt(0, 0, 0);

    this.renderer = new ThreeWebGLRenderer({
      canvas,
      alpha: true,
      antialias: true,
      preserveDrawingBuffer: false
    });
    this.renderer.setSize(canvas.width, canvas.height);
    this.renderer.setPixelRatio(Math.min(window.devicePixelRatio, 2));
    this.renderer.setClearColor(0x000000, 0);

    const ambientLight = new AmbientLight(0x404040, 2);
    this.scene.add(ambientLight);

    this.canvas.style.position = 'fixed';
    this.canvas.style.top = '0';
    this.canvas.style.left = '0';
    this.canvas.style.pointerEvents = 'none';
    this.canvas.style.zIndex = '9999';
    this.canvas.style.width = '100%';
    this.canvas.style.height = '100%';
  }

  /** 加载礼物图片纹理，失败返回 null */
  private async loadGiftTexture(giftId: number | string): Promise<Texture | null> {
    try {
      const url = getGiftImageUrl(giftId);
      return await new Promise((resolve, reject) => {
        this.textureLoader.load(
          url,
          texture => resolve(texture),
          undefined,
          () => reject(new Error(`Failed to load: ${url}`))
        );
      });
    } catch {
      return null;
    }
  }

  /** 创建图片精灵 (始终面朝相机的 billboard) */
  private createGiftSprite(texture: Texture): Sprite {
    const material = new SpriteMaterial({
      map: texture,
      transparent: true,
      blending: NormalBlending,
      depthWrite: false
    });
    const sprite = new Sprite(material);
    sprite.scale.set(256, 256, 1);
    return sprite;
  }

  /** 背景粒子爆发 */
  private createBurstParticles(config: EffectConfig): Points {
    const count = 120;
    const geometry = new BufferGeometry();
    const positions = new Float32Array(count * 3);
    const colors = new Float32Array(count * 3);
    const baseColor = new Color(config.primaryColor);
    const secondary = new Color(config.secondaryColor);

    for (let i = 0; i < count; i++) {
      const angle = Math.random() * Math.PI * 2;
      const phi = Math.random() * Math.PI - Math.PI / 2;
      const radius = 100 + Math.random() * 300;
      positions[i * 3] = Math.cos(angle) * Math.cos(phi) * radius;
      positions[i * 3 + 1] = Math.sin(phi) * radius;
      positions[i * 3 + 2] = Math.cos(angle) * Math.sin(phi) * radius;

      const mixed = baseColor.clone().lerp(secondary, Math.random());
      colors[i * 3] = mixed.r;
      colors[i * 3 + 1] = mixed.g;
      colors[i * 3 + 2] = mixed.b;
    }

    geometry.setAttribute('position', new BufferAttribute(positions, 3));
    geometry.setAttribute('color', new BufferAttribute(colors, 3));

    return new Points(geometry, new PointsMaterial({
      size: 4,
      vertexColors: true,
      blending: AdditiveBlending,
      depthWrite: false,
      transparent: true,
      opacity: 0.9
    }));
  }

  async addEffect(data: EffectTriggerData, config: EffectConfig): Promise<string> {
    const id = `webgl_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
    const group = new Group();

    // 从触发数据中提取 giftId
    const giftId = data.comboInfo?.giftId || data.captainInfo?.giftId || '1';
    const texture = await this.loadGiftTexture(giftId);

    if (texture) {
      const sprite = this.createGiftSprite(texture);
      group.add(sprite);
      this.activeEffects.set(id, {
        id,
        type: data.type,
        group,
        startTime: performance.now(),
        duration: config.duration,
        phase: 'entry',
        phaseStartTime: performance.now(),
        entryDuration: config.duration * 0.3,
        displayDuration: config.duration * 0.5,
        exitDuration: config.duration * 0.2,
        sprite,
        particles: null,
        light: null
      });
    } else {
      // 图片加载失败 — 使用纯色粒子占位
      const particles = this.createBurstParticles(config);
      group.add(particles);
      this.activeEffects.set(id, {
        id,
        type: data.type,
        group,
        startTime: performance.now(),
        duration: config.duration,
        phase: 'entry',
        phaseStartTime: performance.now(),
        entryDuration: config.duration * 0.3,
        displayDuration: config.duration * 0.5,
        exitDuration: config.duration * 0.2,
        sprite: particles,
        particles,
        light: null
      });
    }

    // 粒子爆发
    const particles = this.createBurstParticles(config);
    group.add(particles);

    // 点光源
    const light = new PointLight(config.primaryColor, 2, 400);
    group.add(light);

    // 修正 effect 中的 particles/light 引用
    const effect = this.activeEffects.get(id)!;
    effect.particles = particles;
    effect.light = light;

    group.position.y = -400;
    group.scale.set(0.3, 0.3, 0.3);
    this.scene.add(group);

    if (!this.isRunning) {
      this.isRunning = true;
      this.animate();
    }

    return id;
  }

  removeEffect(id: string): void {
    const effect = this.activeEffects.get(id);
    if (!effect) return;
    this.scene.remove(effect.group);
    this.disposeGroup(effect.group);
    this.activeEffects.delete(id);
    if (this.activeEffects.size === 0) this.isRunning = false;
  }

  clear(): void {
    for (const [id] of this.activeEffects) {
      this.removeEffect(id);
    }
    this.activeEffects.clear();
    this.isRunning = false;
  }

  private animate(): void {
    if (!this.isRunning) return;
    this.animationId = requestAnimationFrame(() => this.animate());

    const now = performance.now();
    const toRemove: string[] = [];

    for (const [id, effect] of this.activeEffects) {
      const elapsed = now - effect.startTime;
      const totalProgress = Math.min(elapsed / effect.duration, 1);
      if (totalProgress >= 1) {
        toRemove.push(id);
        continue;
      }
      this.updateAnimation(effect, totalProgress, now);
    }

    for (const id of toRemove) {
      this.removeEffect(id);
    }

    this.renderer.render(this.scene, this.camera);
  }

  private updateAnimation(effect: ActiveEffect, progress: number, now: number): void {
    const entryRatio = effect.entryDuration / effect.duration;
    const displayRatio = effect.displayDuration / effect.duration;

    let phase: AnimationPhase;
    if (progress < entryRatio) {
      phase = 'entry';
    } else if (progress < entryRatio + displayRatio) {
      phase = 'display';
    } else {
      phase = 'exit';
    }

    if (phase !== effect.phase) {
      effect.phase = phase;
      effect.phaseStartTime = now;
    }

    const group = effect.group;
    const phaseElapsed = now - effect.phaseStartTime;

    switch (phase) {
      case 'entry': {
        const t = Math.min(phaseElapsed / effect.entryDuration, 1);
        const eased = 1 - Math.pow(1 - t, 3);
        group.position.y = -400 + 400 * eased;
        group.scale.setScalar(0.3 + 0.7 * eased);
        group.rotation.y += 0.04;
        break;
      }

      case 'display': {
        group.position.y = Math.sin(now * 0.002) * 15;
        group.scale.setScalar(1);

        // 礼物图片轻微弹跳缩放
        const bounce = 1 + Math.sin(now * 0.008) * 0.05;
        effect.sprite.scale.set(256 * bounce, 256 * bounce, 1);

        // 粒子扩散
        if (effect.particles && effect.particles !== effect.sprite) {
          const pScale = 1 - (progress - entryRatio) / displayRatio;
          effect.particles.scale.setScalar(1 + (1 - pScale) * 0.4);
          (effect.particles.material as PointsMaterial).opacity = 0.9 * pScale;
        }

        if (effect.light) {
          effect.light.intensity = 2 + Math.sin(now * 0.005) * 1.5;
        }
        break;
      }

      case 'exit': {
        const p = Math.min(phaseElapsed / effect.exitDuration, 1);
        group.scale.setScalar(1 - p * 0.7);
        group.position.y += 2;

        // 图片淡出
        const spriteMaterial = (effect.sprite as Sprite | Points).material;
        if (spriteMaterial instanceof SpriteMaterial) {
          spriteMaterial.opacity = 1 - p;
        } else if (spriteMaterial instanceof PointsMaterial) {
          spriteMaterial.opacity = Math.max(0, 0.9 * (1 - p));
        }

        if (effect.particles && effect.particles !== effect.sprite) {
          (effect.particles.material as PointsMaterial).opacity = Math.max(0, 0.9 * (1 - p));
        }
        if (effect.light) {
          effect.light.intensity = 2 * (1 - p);
        }
        break;
      }
    }
  }

  private disposeGroup(group: Group): void {
    group.traverse((child) => {
      if (child instanceof Mesh || child instanceof Points || child instanceof Line) {
        child.geometry.dispose();
        const mat = child.material;
        if (Array.isArray(mat)) {
          mat.forEach(m => m.dispose());
        } else if (mat instanceof Material) {
          mat.dispose();
        }
      }
      if (child instanceof Light && child.parent) {
        child.parent.remove(child);
      }
    });
  }

  resize(width: number, height: number): void {
    this.renderer.setSize(width, height);
    this.camera.aspect = width / height;
    this.camera.updateProjectionMatrix();
  }

  getActiveCount(): number {
    return this.activeEffects.size;
  }

  dispose(): void {
    this.clear();
    cancelAnimationFrame(this.animationId);
    this.renderer.dispose();
    this.scene.clear();
  }
}
