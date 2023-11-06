import { ReactNode, createContext, useContext, useMemo, useState } from "react";

export enum AuthState {
    Unauthenticated,
    Authenticated,
    AuthInProgress,
}

type Actions = {
    setAuthenticated: () => void;
    setUnauthenticated: () => void;
    setAuthInProgress: () => void;
};

const AuthContextData = createContext<AuthState>(AuthState.Unauthenticated);
const AuthContextActions = createContext<Actions>({} as Actions);

type Props = {
    children: ReactNode;
};

function AuthContext({ children }: Props) {
    const [state, setState] = useState<AuthState>(AuthState.Unauthenticated);
    console.log(state);
    const actions = useMemo(
        () => ({
            setAuthenticated: () => setState(AuthState.Authenticated),
            setUnauthenticated: () => setState(AuthState.Unauthenticated),
            setAuthInProgress: () => setState(AuthState.AuthInProgress),
        }),
        []
    );

    return (
        <AuthContextActions.Provider value={actions}>
            <AuthContextData.Provider value={state}>
                {children}
            </AuthContextData.Provider>
        </AuthContextActions.Provider>
    );
}

export const useAuthContextData = () => useContext(AuthContextData);
export const useAuthContextActions = () => useContext(AuthContextActions);

export default AuthContext;
