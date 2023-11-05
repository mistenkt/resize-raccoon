import { Profile } from "../types/ProfileTypes"
import { useEffect, useState } from "react"
import ProfileListItem from "./ProfileListItem";
import { FilePlus } from "react-feather";
import backend from "../utils/backend";

interface Props {
    onNewProfile: () => void;
    onEditProfile: (profile: Profile) => void;
}

const ProfileList = ({onNewProfile, onEditProfile}: Props) => {
    const [profiles, setProfiles] = useState<Profile[]>([]);

    useEffect(() => {
        backend.profile.all().then(setProfiles);
    }, []);

    const handleResize = (profile: Profile) => {
        backend.profile.apply(profile);
    }

    const dummyProfiles = [...profiles, ...profiles, ...profiles, ...profiles, ...profiles, ...profiles, ...profiles];

    return (
        <div className="flex flex-col h-full overflow-y-scroll pr-8 pt-12 pb-12">
            {dummyProfiles.map((p, i) => (
                <ProfileListItem 
                    key={i}
                    profile={p}
                    onEdit={onEditProfile}
                    onResize={handleResize}
                />
            ))}
        </div>
    )
}
export default ProfileList;