import styled from '@emotion/styled';
import { Profile } from '../types/ProfileTypes';
import { Maximize, Settings } from 'react-feather';
import { useState } from 'react';
import backend from '../utils/backend';
import { setScreen } from '../state/screenState';
import { Screen } from '../types/ScreenTypes';
import { useTranslation } from '../utils/i18n/useTranslation';

interface Props {
    profile: Profile;
}

const ProfileListItem = ({ profile }: Props) => {
    const t = useTranslation();
    const [loading, setLoading] = useState(false);

    const handleResize = () => {
        setLoading(true);
        backend.profile.apply(profile).finally(() => setLoading(false));
    };

    const handleEditProfile = () => {
        setScreen(Screen.PROFILE_EDITOR, { profile });
    };

    return (
        <Component className="card card-compact shadow-xl bg-base-100 mb-2">
            <div className="card-body gap-y-0">
                <div className="row">
                    <div className="label text-lg p-0">{profile.name}</div>
                    <div className="actions gap-1">
                        <button
                            disabled={loading}
                            className="btn btn-ghost btn-square btn-sm"
                            onClick={handleResize}
                        >
                            {loading ? (
                                <span className="loading loading-spinner w-4" />
                            ) : (
                                <Maximize size={16} />
                            )}
                        </button>
                        <button
                            className="btn btn-ghost btn-square btn-sm"
                            onClick={handleEditProfile}
                        >
                            <Settings size={16} />
                        </button>
                    </div>
                </div>
                <div className="meta text-2xs uppercase font-bold text-gray-600">
                    <span>
                        {profile.auto
                            ? t('profile.autoResize.enabled')
                            : t('profile.autoResize.disabled')}
                    </span>
                    <span>w: {profile.windowWidth}</span>
                    <span>h: {profile.windowHeight}</span>
                    <span>x: {profile.windowPosX}</span>
                    <span>y: {profile.windowPosY}</span>
                </div>
            </div>
        </Component>
    );
};

const Component = styled.div`
    .row {
        display: flex;
        flex-direction: row;
        width: 100%;
    }

    .label {
        display: flex;
        flex: 1;
    }

    .meta {
        display: inline-block;
        display: flex;
        gap: 5px;
        justify-content: flex-start;
        line-height: 1em;

        // add a bullet between each item
        span:not(:last-child)::after {
            content: '•';
            margin-left: 5px;
        }
    }

    .actions {
        display: flex;
        align-items: center;
    }
`;

export default ProfileListItem;
