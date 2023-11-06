import { useState } from "react";
import {
    AuthState,
    useAuthContextActions,
    useAuthContextData,
} from "../context/AuthContext";

type FormInput = {
    value: string;
    label: string;
    type: "text" | "password";
};

type CredentialForm = {
    email: FormInput;
    password: FormInput;
    confirmedPassword?: FormInput;
};

const baseLoginForm: CredentialForm = {
    email: {
        value: "",
        label: "Email",
        type: "text",
    },
    password: {
        value: "",
        label: "Password",
        type: "password",
    },
};

const baseSignUpForm: CredentialForm = {
    ...baseLoginForm,
    confirmedPassword: {
        value: "",
        label: "Confirm password",
        type: "password",
    },
};

function Login() {
    const [form, setForm] = useState<CredentialForm>(baseLoginForm);
    const [passwordMatch, setPasswordMatch] = useState(true);
    const auth = useAuthContextData();
    const { setAuthenticated } = useAuthContextActions();
    let isSignUp = form.confirmedPassword !== undefined;

    const submit = () => {
        if (isSignUp && form.password !== form.confirmedPassword) {
            setPasswordMatch(false);
            return;
        }
        setAuthenticated();
    };

    const changeForm = () => {
        if (isSignUp) {
            let newForm = { ...form };
            delete newForm.confirmedPassword;
            setForm(newForm);
            setPasswordMatch(true);
        } else {
            setForm((prev) => ({
                ...prev,
                confirmedPassword: baseSignUpForm.confirmedPassword,
            }));
        }
    };

    const formFields = Object.keys(form) as Array<keyof CredentialForm>;

    return (
        <dialog
            open={auth === AuthState.AuthInProgress}
            className="p-4 rounded-xl bg-green-secondary"
        >
            <div className="flex-col min-h-96 w-96">
                <form method="dialog" className="flex-col justify-center">
                    {formFields.map((key) => (
                        <div key={key} className="pb-4">
                            <input
                                className="w-full rounded px-2 py-1"
                                placeholder={form[key]?.label}
                                type={form[key]?.type}
                                value={form[key]?.value}
                                onChange={(e) =>
                                    setForm((prev) => ({
                                        ...prev,
                                        [key]: {
                                            ...form[key],
                                            value: e.target.value,
                                        },
                                    }))
                                }
                            />
                        </div>
                    ))}

                    <div className="pb-4">
                        <button
                            type="button"
                            onClick={() => submit()}
                            className="h-12 w-full rounded-lg bg-yellow-primary hover:bg-orange-primary"
                        >
                            <span className="inline-block align-middle text-green-primary">
                                {isSignUp ? "Create account" : "Sign in"}
                            </span>
                        </button>
                    </div>
                </form>
                <div className="flex justify-center">
                    <button onClick={changeForm} className="">
                        <span className="inline-block align-middle text-white hover:text-yellow-primary">
                            {isSignUp
                                ? "Already have an account?"
                                : "Create an account"}
                        </span>
                    </button>
                </div>
            </div>
        </dialog>
    );
}

export default Login;
