import vegetable from "../assets/vegetables_64.png";
import { AuthState, useAuthContextData } from "../context/AuthContext";
import Login from "./Login";
import LoginButton from "./LoginButton";
import Recipe from "./Recipe";

function TopMenu() {
    const auth = useAuthContextData();

    return (
        <>
            <div className="flex justify-between bg-green-primary">
                <div className="p-4 flex gap-4">
                    <img src={vegetable} className="content-center"></img>
                    <span className="text-4xl self-center self-justify-center text-white font-medium">
                        recip<span className="text-yellow-primary">easy</span>
                    </span>
                </div>
                <div className="self-center self-justify-center">
                    {auth !== AuthState.AuthInProgress && <LoginButton />}
                </div>
            </div>
            {auth === AuthState.AuthInProgress && (
                <div className="flex items-center justify-center h-screen">
                    <Login />
                </div>
            )}
            <Recipe />
        </>
    );
}

export default TopMenu;
