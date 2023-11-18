import './utils/i18n/i18n';
import ProfileEditor from "./screens/ProfileEditor";
import HomeScreen from "./screens/HomeScreen";
import ToastProvider from "./components/toast/ToastSystem";
import {Screen} from "./types/ScreenTypes";
import { BootState, getBootState } from "./state/bootState";
import { getScreen, getScreenParams } from "./state/screenState";

const routes: Record<Screen, (props: any) => JSX.Element> = {
    [Screen.HOME]: HomeScreen,
    [Screen.PROFILE_EDITOR]: ProfileEditor,
}

function App() {
    //TODO: Display a splash screen before state is ready
    if(getBootState() !== BootState.READY) return null;
    const ActiveScreen = routes[getScreen()];
    

    return (
        <>
            <ActiveScreen {...getScreenParams()}/>
            <ToastProvider/>
        </>
    )
}

export default App;
