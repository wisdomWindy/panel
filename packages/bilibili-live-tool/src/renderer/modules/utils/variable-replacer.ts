/**
 * 变量替换工具
 * 支持基础变量、嵌套属性、动态变量、递归替换
 */

/**
 * 变量替换配置
 */
export interface ReplacerConfig {
  /**
   * 变量前缀
   * @default '{'
   */
  prefix?: string;

  /**
   * 变量后缀
   * @default '}'
   */
  suffix?: string;

  /**
   * 是否递归替换
   * @default false
   */
  recursive?: boolean;

  /**
   * 是否严格模式（未定义变量时抛出错误）
   * @default false
   */
  strict?: boolean;

  /**
   * 变量上下文
   */
  context?: Record<string, unknown>;
}

/**
 * 变量替换器
 */
export class VariableReplacer {
  private config: Required<ReplacerConfig>;

  constructor(config: ReplacerConfig = {}) {
    this.config = {
      prefix: config.prefix || '{',
      suffix: config.suffix || '}',
      recursive: config.recursive || false,
      strict: config.strict || false,
      context: config.context || {}
    };
  }

  /**
   * 替换字符串中的变量
   * @param template 模板字符串
   * @param context 变量上下文
   * @returns 替换后的字符串
   */
  replace(template: string, context?: Record<string, unknown>): string {
    const ctx = { ...this.config.context, ...context };

    if (this.config.recursive) {
      return this.replaceRecursive(template, ctx);
    } else {
      return this.replaceSingle(template, ctx);
    }
  }

  /**
   * 单次替换
   */
  private replaceSingle(template: string, context: Record<string, unknown>): string {
    const regex = new RegExp(
      `\\${this.config.prefix}([^\\${this.config.prefix}\\${this.config.suffix}]+)\\${this.config.suffix}`,
      'g'
    );

    return template.replace(regex, (match, variable) => {
      try {
        const value = this.getValue(variable, context);
        return this.formatValue(value);
      } catch (error) {
        if (this.config.strict) {
          throw error;
        }
        return match;
      }
    });
  }

  /**
   * 递归替换
   */
  private replaceRecursive(template: string, context: Record<string, unknown>): string {
    let result = template;
    let previousResult = '';

    // 最多递归 10 次，防止死循环
    const maxIterations = 10;
    let iterations = 0;

    while (result !== previousResult && iterations < maxIterations) {
      previousResult = result;
      result = this.replaceSingle(result, context);
      iterations++;
    }

    return result;
  }

  /**
   * 获取变量值
   */
  private getValue(variable: string, context: Record<string, unknown>): unknown {
    // 支持嵌套属性访问，如 {user.name}
    const keys = variable.split('.');
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    let value: any = context;

    for (const key of keys) {
      if (value === null || value === undefined) {
        throw new Error(`Variable "${variable}" is undefined`);
      }

      if (typeof value === 'object' && key in value) {
        value = value[key];
      } else {
        throw new Error(`Variable "${variable}" is undefined`);
      }
    }

    // 如果值是函数，执行它
    if (typeof value === 'function') {
      return value(context);
    }

    return value;
  }

  /**
   * 格式化值
   */
  private formatValue(value: unknown): string {
    if (value === null || value === undefined) {
      return '';
    }

    if (typeof value === 'object') {
      return JSON.stringify(value);
    }

    return String(value);
  }

  /**
   * 批量替换
   * @param templates 模板数组
   * @param context 变量上下文
   * @returns 替换后的字符串数组
   */
  replaceBatch(templates: string[], context?: Record<string, unknown>): string[] {
    return templates.map(template => this.replace(template, context));
  }

  /**
   * 提取所有变量
   * @param template 模板字符串
   * @returns 变量列表
   */
  extractVariables(template: string): string[] {
    const regex = new RegExp(
      `\\${this.config.prefix}([^\\${this.config.prefix}\\${this.config.suffix}]+)\\${this.config.suffix}`,
      'g'
    );

    const variables = new Set<string>();
    let match;

    while ((match = regex.exec(template)) !== null) {
      variables.add(match[1]);
    }

    return Array.from(variables);
  }

  /**
   * 验证变量是否完整
   * @param template 模板字符串
   * @param context 变量上下文
   * @returns 是否完整
   */
  validate(
    template: string,
    context?: Record<string, unknown>
  ): {
    valid: boolean;
    missing: string[];
  } {
    const ctx = { ...this.config.context, ...context };
    const variables = this.extractVariables(template);
    const missing: string[] = [];

    for (const variable of variables) {
      try {
        this.getValue(variable, ctx);
      } catch {
        missing.push(variable);
      }
    }

    return {
      valid: missing.length === 0,
      missing
    };
  }

  /**
   * 设置上下文
   */
  setContext(context: Record<string, unknown>): void {
    this.config.context = context;
  }

  /**
   * 获取上下文
   */
  getContext(): Record<string, unknown> {
    return { ...this.config.context };
  }

  /**
   * 清空上下文
   */
  clearContext(): void {
    this.config.context = {};
  }
}

/**
 * 便捷函数：替换字符串中的变量
 */
export function replaceVariables(
  template: string,
  context: Record<string, unknown>,
  config?: ReplacerConfig
): string {
  const replacer = new VariableReplacer(config);
  return replacer.replace(template, context);
}

/**
 * 便捷函数：批量替换
 */
export function replaceVariablesBatch(
  templates: string[],
  context: Record<string, unknown>,
  config?: ReplacerConfig
): string[] {
  const replacer = new VariableReplacer(config);
  return replacer.replaceBatch(templates, context);
}

/**
 * 便捷函数：提取变量
 */
export function extractVariables(template: string, config?: ReplacerConfig): string[] {
  const replacer = new VariableReplacer(config);
  return replacer.extractVariables(template);
}
