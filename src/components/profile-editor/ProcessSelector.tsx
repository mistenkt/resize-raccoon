import { useEffect, useState } from "react";
import Process from "../../types/ProcessType";
import backend from "../../utils/backend";
import LoadingButton from "../LoadingButton";
import { RefreshCw } from "react-feather";

interface Props {
    selectedProcess?: Process;
    processNameValue?: string;
    onChange: (process: Process) => void;
}

const ProcessSelector = ({ selectedProcess, processNameValue, onChange }: Props) => {
    const [processes, setProcesses] = useState<Process[]>([]);

    useEffect(() => {
        backend.process.running().then(setProcesses);
    }, []);

    const handleReloadProcesses = (stopLoading: () => void) => {
        backend.process.running().then(setProcesses).finally(stopLoading);
    };

    return (
        <div className="flex gap-2">
            <select
                id="process"
                className="select w-full"
                onChange={(e) => {
                    onChange(processes.find(
                        (p) => p.pid === parseInt(e.target.value)
                    )!)
                }}
                value={selectedProcess?.pid}
                defaultValue="default"
            >
                {processNameValue && !selectedProcess ? (
                    <option value="default">{processNameValue}</option>
                ) : (
                    <option value="default">Select a process</option>
                )}

                {processes.map((p) => (
                    <option key={p.pid} value={p.pid}>
                        {p.name}
                    </option>
                ))}
            </select>
            <LoadingButton
                className="btn btn-outline"
                onClick={handleReloadProcesses}
                onlySpinner
            >
                <RefreshCw size={16} />
            </LoadingButton>
        </div>
    );
};

export default ProcessSelector;