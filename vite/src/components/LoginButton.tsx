import {
    AuthState,
    useAuthContextActions,
    useAuthContextData,
} from "../context/AuthContext";

function LoginButton() {
    const { setAuthInProgress, setUnauthenticated } = useAuthContextActions();
    const auth = useAuthContextData();

    const updateAuth = () => {
        switch (auth) {
            case AuthState.AuthInProgress: {
                console.log("Error");
                break;
            }
            case AuthState.Authenticated: {
                setUnauthenticated();
                break;
            }
            case AuthState.Unauthenticated: {
                setAuthInProgress();
                break;
            }
        }
    };

    return (
        <button
            onClick={updateAuth}
            className="m-4 h-12 w-24 rounded-lg bg-green-secondary hover:bg-yellow-primary text-white hover:text-green-primary"
        >
            <span className="inline-block align-middle">
                {auth === AuthState.Unauthenticated ? "Sign in" : "Sign out"}
            </span>
        </button>
    );
}

export default LoginButton;
