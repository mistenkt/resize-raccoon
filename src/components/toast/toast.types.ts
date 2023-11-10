export enum ToastType {
    SUCCESS = 'success',
    ERROR = 'error',
    INFO = 'info',
    WARNING = 'warning'
}

export interface Toast {
    type: ToastType;
    message: string;
    hideAfterMs?: number;
}

export interface ToastContextData {
    addToast(toast: Toast): void;
    removeToast(): void;
}