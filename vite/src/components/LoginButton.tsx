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
            className="m-4 h-12 w-24 rounded-lg duration-200 bg-green-secondary hover:bg-yellow-primary text-white hover:text-green-primary shadow-sm hover:shadow-md"
        >
            <span className="inline-block align-middle">
                {auth === AuthState.Unauthenticated ? "Sign in" : "Sign out"}
            </span>
        </button>
    );
}

export default LoginButton;
