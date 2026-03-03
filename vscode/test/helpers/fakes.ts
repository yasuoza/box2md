import type { EditorLike, SelectionLike } from '../../src/editorIO';
import type { NotificationApi } from '../../src/notifications';
import type { ClipboardLike } from '../../src/output';

export class FakeEditor implements EditorLike {
  private value: string;
  private currentSelection: SelectionLike;

  constructor(content: string, selection?: SelectionLike) {
    this.value = content;
    this.currentSelection = selection ?? {
      start: 0,
      end: 0,
      isEmpty: true
    };
  }

  get content(): string {
    return this.value;
  }

  setSelection(start: number, end: number): void {
    this.currentSelection = {
      start,
      end,
      isEmpty: start === end
    };
  }

  get selection(): SelectionLike {
    return this.currentSelection;
  }

  getText(range?: SelectionLike): string {
    if (!range) {
      return this.value;
    }
    return this.value.slice(range.start, range.end);
  }

  async replaceSelection(text: string): Promise<void> {
    const { start, end } = this.currentSelection;
    this.value = `${this.value.slice(0, start)}${text}${this.value.slice(end)}`;
    const cursor = start + text.length;
    this.setSelection(cursor, cursor);
  }

  async insertAtCursor(text: string): Promise<void> {
    const cursor = this.currentSelection.end;
    this.value = `${this.value.slice(0, cursor)}${text}${this.value.slice(cursor)}`;
    const nextCursor = cursor + text.length;
    this.setSelection(nextCursor, nextCursor);
  }
}

export class FakeClipboard implements ClipboardLike {
  public text = '';

  async writeText(text: string): Promise<void> {
    this.text = text;
  }
}

export class FakeNotifications implements NotificationApi {
  public readonly infos: string[] = [];
  public readonly errors: string[] = [];

  async showInformationMessage(message: string): Promise<string | undefined> {
    this.infos.push(message);
    return undefined;
  }

  async showErrorMessage(message: string): Promise<string | undefined> {
    this.errors.push(message);
    return undefined;
  }
}
