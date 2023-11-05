import styled from "@emotion/styled";
import { Profile } from "../types/ProfileTypes";
import { Maximize, Settings } from "react-feather";

interface Props {
    profile: Profile;
    onEdit: (profile: Profile) => void;
    onResize: (profile: Profile) => void;
}

const ProfileListItem = ({profile, onResize, onEdit}: Props) => {
    return (
        <Component className="card card-compact shadow-xl bg-base-100 mb-2">
            <div className="card-body gap-y-0">
                <div className="row">
                    <div className="label text-lg p-0">{profile.name}</div>
                    <div className="actions gap-1">
                        
                        <button onClick={() => onResize(profile)}>
                            <Maximize size={16}/>
                        </button>
                        <button onClick={() => onEdit(profile)}>
                            <Settings size={16}/>
                        </button>
                    </div>
                </div>
                <div className="meta text-2xs uppercase font-bold text-gray-600">
                    <span>{profile.auto ? 'automatic' : 'manual'}</span>
                    <span>w: {profile.windowWidth}</span>
                    <span>h: {profile.windowHeight}</span>
                    <span>x: {profile.windowPosX}</span>
                    <span>y: {profile.windowPosY}</span>
                </div>
            </div>
            
            
        </Component>
    )
}

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

        button {
            all: unset;
            width: 24px;
            height: 24px;
            display: inline-flex;
            align-items: center;
            justify-content: center;
            cursor: pointer;

            &:hover svg {
                transform: scale(1.2);
                transition: all 0.2s ease;
            }
        }
    }
`;

export default ProfileListItem;