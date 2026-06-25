/**
 * 敏感词过滤工具
 * 使用 Trie 树实现高效敏感词检测和过滤
 * 支持 Unicode（包括中文）
 */

/**
 * Trie 树节点
 */
interface TrieNode {
  children: Map<string, TrieNode>;
  isEnd: boolean;
  depth: number;
}

/**
 * 敏感词配置
 */
export interface SensitiveWordsConfig {
  /**
   * 替换字符
   * @default '*'
   */
  replaceChar?: string;

  /**
   * 是否区分大小写
   * @default false
   */
  caseSensitive?: boolean;

  /**
   * 是否检测单词边界
   * @default false
   */
  wordBoundary?: boolean;
}

/**
 * 检测结果
 */
export interface DetectResult {
  matched: boolean;
  words: string[];
  positions: Array<{ start: number; end: number; word: string }>;
}

/**
 * 敏感词过滤器
 */
export class SensitiveWordsFilter {
  private root: TrieNode;
  private config: Required<SensitiveWordsConfig>;
  private wordList: Set<string>;

  constructor(config: SensitiveWordsConfig = {}) {
    this.root = this.createNode();
    this.config = {
      replaceChar: config.replaceChar || '*',
      caseSensitive: config.caseSensitive || false,
      wordBoundary: config.wordBoundary || false
    };
    this.wordList = new Set();
  }

  /**
   * 创建 Trie 节点
   */
  private createNode(): TrieNode {
    return {
      children: new Map(),
      isEnd: false,
      depth: 0
    };
  }

  /**
   * 添加敏感词
   * @param word 敏感词
   */
  addWord(word: string): void {
    if (!word || word.length === 0) {
      return;
    }

    let node = this.root;
    const chars = this.tokenize(word);

    for (let i = 0; i < chars.length; i++) {
      const char = chars[i];

      if (!node.children.has(char)) {
        const newNode = this.createNode();
        newNode.depth = node.depth + 1;
        node.children.set(char, newNode);
      }

      const nextNode = node.children.get(char);
      if (!nextNode) break;
      node = nextNode;
    }

    node.isEnd = true;
    this.wordList.add(word);
  }

  /**
   * 批量添加敏感词
   * @param words 敏感词列表
   */
  addWords(words: string[]): void {
    for (const word of words) {
      this.addWord(word);
    }
  }

  /**
   * 移除敏感词
   * @param word 敏感词
   */
  removeWord(word: string): void {
    if (!this.wordList.has(word)) {
      return;
    }

    this.wordList.delete(word);

    // 重建 Trie 树（简化实现）
    const words = Array.from(this.wordList);
    this.clear();
    this.addWords(words);
  }

  /**
   * 批量移除敏感词
   * @param words 敏感词列表
   */
  removeWords(words: string[]): void {
    for (const word of words) {
      this.removeWord(word);
    }
  }

  /**
   * 清空所有敏感词
   */
  clear(): void {
    this.root = this.createNode();
    this.wordList.clear();
  }

  /**
   * 检测文本中是否包含敏感词
   * @param text 待检测文本
   * @returns 检测结果
   */
  detect(text: string): DetectResult {
    const positions: Array<{ start: number; end: number; word: string }> = [];
    const words = new Set<string>();

    const chars = this.tokenize(text);

    for (let i = 0; i < chars.length; i++) {
      let node = this.root;
      const matchStart = i;
      let matchEnd = i;

      for (let j = i; j < chars.length; j++) {
        const char = chars[j];

        if (!node.children.has(char)) {
          break;
        }

        const nextNode = node.children.get(char);
        if (!nextNode) break;
        node = nextNode;
        matchEnd = j;

        if (node.isEnd) {
          const word = this.untokenize(chars.slice(matchStart, matchEnd + 1));
          words.add(word);
          positions.push({
            start: matchStart,
            end: matchEnd + 1,
            word
          });

          if (this.config.wordBoundary) {
            break;
          }
        }
      }
    }

    return {
      matched: positions.length > 0,
      words: Array.from(words),
      positions
    };
  }

  /**
   * 过滤敏感词
   * @param text 待过滤文本
   * @returns 过滤后的文本
   */
  filter(text: string): string {
    const result = this.detect(text);

    if (!result.matched) {
      return text;
    }

    const chars = this.tokenize(text);

    // 创建替换标记数组
    const replaceFlags = new Array(chars.length).fill(false);

    for (const pos of result.positions) {
      for (let i = pos.start; i < pos.end; i++) {
        replaceFlags[i] = true;
      }
    }

    // 执行替换
    const filteredChars = chars.map((char, index) => {
      return replaceFlags[index] ? this.config.replaceChar : char;
    });

    return this.untokenize(filteredChars);
  }

  /**
   * 获取敏感词列表
   * @returns 敏感词列表
   */
  getWords(): string[] {
    return Array.from(this.wordList);
  }

  /**
   * 获取敏感词数量
   * @returns 敏感词数量
   */
  getWordCount(): number {
    return this.wordList.size;
  }

  /**
   * 检查是否包含某个敏感词
   * @param word 敏感词
   * @returns 是否包含
   */
  hasWord(word: string): boolean {
    return this.wordList.has(word);
  }

  /**
   * 文本分词（支持 Unicode）
   */
  private tokenize(text: string): string[] {
    const result: string[] = [];
    let i = 0;

    while (i < text.length) {
      const charCode = text.charCodeAt(i);

      if (charCode < 0xd800 || charCode > 0xdbff) {
        // 基本多文种平面
        const char = this.config.caseSensitive ? text[i] : text[i].toLowerCase();
        result.push(char);
        i++;
      } else {
        // 代理对（Emoji 等）
        if (i + 1 < text.length) {
          const char = this.config.caseSensitive
            ? text.substring(i, i + 2)
            : text.substring(i, i + 2).toLowerCase();
          result.push(char);
          i += 2;
        } else {
          break;
        }
      }
    }

    return result;
  }

  /**
   * 文本还原（从分词数组还原）
   */
  private untokenize(tokens: string[]): string {
    return tokens.join('');
  }
}

/**
 * 便捷函数：检测敏感词
 */
export function detectSensitiveWords(
  text: string,
  words: string[],
  config?: SensitiveWordsConfig
): DetectResult {
  const filter = new SensitiveWordsFilter(config);
  filter.addWords(words);
  return filter.detect(text);
}

/**
 * 便捷函数：过滤敏感词
 */
export function filterSensitiveWords(
  text: string,
  words: string[],
  config?: SensitiveWordsConfig
): string {
  const filter = new SensitiveWordsFilter(config);
  filter.addWords(words);
  return filter.filter(text);
}

/**
 * 便捷函数：批量检测敏感词
 */
export function detectSensitiveWordsBatch(
  texts: string[],
  words: string[],
  config?: SensitiveWordsConfig
): DetectResult[] {
  const filter = new SensitiveWordsFilter(config);
  filter.addWords(words);
  return texts.map(text => filter.detect(text));
}

/**
 * 便捷函数：批量过滤敏感词
 */
export function filterSensitiveWordsBatch(
  texts: string[],
  words: string[],
  config?: SensitiveWordsConfig
): string[] {
  const filter = new SensitiveWordsFilter(config);
  filter.addWords(words);
  return texts.map(text => filter.filter(text));
}
