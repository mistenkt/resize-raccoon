import { useTranslation } from "../../utils/i18n/useTranslation";
import LoadingButton from "../LoadingButton";

interface Props {
    onTest: (stopLoading:() => void) => void;
    canTest: boolean;
    onDelete?: () => void;
    onCancel: () => void;
    onSave: () => void;
    canSave?: boolean;
}

const ProfileEditorFooter = ({ onTest, canTest, onDelete, onCancel, onSave, canSave = true }: Props) => {
    const t = useTranslation();

    return (
        <div className="pb-8 pr-8 pl-8">
            <div className="divider pt-0 mt-0"></div>
            <div className="grid grid-cols-3 gap-4">
                <div className="text-left">
                    <LoadingButton
                        className="btn btn-outline btn-info"
                        onClick={onTest}
                        disabled={!canTest}
                    >
                        {t('profile.buttons.test')}
                    </LoadingButton>
                </div>
                <div className="text-center">
                    {!!onDelete && (
                        <LoadingButton
                            className="btn btn-outline btn-secondary mr-4"
                            onClick={onDelete}
                        >
                            {t('profile.buttons.delete')}
                        </LoadingButton>
                    )}
                    <button
                        className="btn btn-outline"
                        onClick={onCancel}
                    >
                        {t('profile.buttons.cancel')}
                    </button>
                </div>
                <div className="text-right">
                    <LoadingButton
                        className="btn"
                        onClick={onSave}
                        disabled={!canSave}
                    >
                        {t('profile.buttons.save')}
                    </LoadingButton>
                </div>
            </div>
        </div>
    )
}

export default ProfileEditorFooter;