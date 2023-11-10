import { useState } from "react";

interface Props extends Omit<React.ButtonHTMLAttributes<HTMLButtonElement>, 'onClick'> {
    onClick: (stopLoading: () => void) => void;
    onlySpinner?: boolean;
}

const LoadingButton = ({onClick, onlySpinner, children, ...props}: Props) => {
    const [loading, setLoading] = useState(false);

    const handleClick = () => {
        setLoading(true);
        onClick(() => setLoading(false));
    }

    return (
        <button onClick={handleClick} {...props}>
            {!(onlySpinner && loading) && children}
            {loading && (
                <span className="loading loading-spinner w-4"/>
            )}
        </button>
    )
}

export default LoadingButton;