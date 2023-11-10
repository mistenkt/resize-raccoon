import { useMemo } from "react";
import { Info } from "react-feather";

interface Props {
    label: string;
    id: string;
    description?: string;
    tooltip?: 'top-left' | 'top-center' | 'top-right' | 'bottom-left' | 'bottom-center' | 'bottom-right';
    children: React.ReactNode;
    className?: string;
}

const FormControl = ({ label, id, description, tooltip = 'top-right', className, children }: Props) => {
    const tooltipClass = useMemo(() => {
        if(!tooltip) return '';
        switch(tooltip) {
            case 'top-center':
                return 'tooltip before:w-[300px]';
            case 'top-left':
                return 'tooltip before:w-[300px] before:left-auto before:right-0 before:translate-x-0';
            case 'top-right':
            default:
                return "tooltip before:w-[300px] before:-left-[10px] before:translate-x-0";

        }
    }, [tooltip]);
    return (
        <div className={`form-control w-full ${className || ''}`}>
            <label
                htmlFor={id}
                className="label pb-1 justify-start gap-2"
            >
                <span className="text-2xs uppercase font-semibold">
                    {label}
                </span>
                {!!description && (
                    <div
                        className={tooltipClass}
                        data-tip={description}
                    >
                        <Info size=".8em" />
                    </div>
                )}
                
            </label>
            {children}
        </div>
    )
}

export default FormControl;