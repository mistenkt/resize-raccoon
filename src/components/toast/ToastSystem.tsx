import { useEffect, useRef, useState } from 'react';
import toastIcons from './toast.icons';
import { Toast, ToastType } from './toast.types';
import { signal } from '@preact/signals-react';
import { produce } from 'immer';
import { useTranslation } from '../../utils/i18n/useTranslation';

interface IncrementalToast extends Toast {
    id: number;
}

const nextId = signal(0);
const toasts = signal<IncrementalToast[]>([]);

export const addToast = (toast: Toast) => {
    toasts.value = produce(toasts.value, (draft) => {
        const newToast = { ...toast, id: nextId.value } as IncrementalToast;
        nextId.value += 1;
        draft.unshift(newToast);
    });
};

export const removeToast = (id: number) => {
    toasts.value = produce(toasts.value, (draft) =>
        draft.filter((t) => t.id !== id)
    );
};

const TOAST_ANIMATION_DURATION = 500;

const ToastSystem = () => {
    const [slideOutToast, setSlideOutToast] = useState<number | null>(null);
    const [slideInToast, setSlideInToast] = useState<number | null>(null);
    const previousLengthRef = useRef(toasts.value.length);
    const t = useTranslation();

    useEffect(() => {
        const toast = toasts.value[0];
        let timer: ReturnType<typeof setTimeout> | null = null;

        if (
            toast &&
            toast.type !== ToastType.ERROR &&
            toast.hideAfterMs !== 0
        ) {
            timer = setTimeout(() => {
                setSlideOutToast(toast.id);
                setTimeout(() => {
                    removeToast(toast.id);
                    setSlideOutToast(null);
                }, TOAST_ANIMATION_DURATION);
            }, toast.hideAfterMs || 3000);
        }

        return () => {
            if (timer) {
                clearTimeout(timer);
            }
        };
    }, [toasts.value]);

    useEffect(() => {
        const toastWasAdded = toasts.value.length > previousLengthRef.current;
        let timer: ReturnType<typeof setTimeout> | null = null;

        if (toastWasAdded) {
            setSlideInToast(toasts.value[0].id);

            timer = setTimeout(() => {
                setSlideInToast(null);
            }, TOAST_ANIMATION_DURATION);
        }

        previousLengthRef.current = toasts.value.length;

        return () => {
            if (timer) {
                clearTimeout(timer);
            }
        };
    }, [toasts.value.length]);

    const dismissToast = () => {
        setSlideOutToast(toasts.value[0].id);
        setTimeout(() => {
            removeToast(toasts.value[0].id);
            setSlideOutToast(null);
        }, TOAST_ANIMATION_DURATION);
    };

    if (!toasts.value.length) return null;

    return (
        <div
            className={`toast-system fixed z-[10] stack bottom-0 p-2 w-full ${
                slideOutToast !== null ? 'removing' : ''
            }`}
        >
            {toasts.value.map((toast) => (
                <div
                    key={toast.id}
                    className={`alert shadow-md alert-${toast.type} ${
                        slideInToast === toast.id ? 'slide-in' : ''
                    } ${slideOutToast === toast.id ? 'slide-out' : ''}`}
                >
                    {toastIcons[toast.type]}
                    <span>{toast.message}</span>
                    <button
                        onClick={dismissToast}
                        className="btn btn-ghost text-base-100 btn-sm self-end"
                    >
                        {t('toast.buttons.dismiss')}
                    </button>
                </div>
            ))}
        </div>
    );
};

export default ToastSystem;
