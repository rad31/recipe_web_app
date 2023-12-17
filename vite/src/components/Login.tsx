import { useState } from "react";
import {
    AuthState,
    useAuthContextActions,
    useAuthContextData,
} from "../context/AuthContext";
import axios from "axios";

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

type AuthRequest = {
    username: string;
    password: string;
};

type FormError = {
    message: string;
};

const DuplicateUserError: FormError = {
    message: "Email already in use",
};

const InvalidCredentialError: FormError = {
    message: "Invalid email or password",
};

const PasswordNotMatchingError: FormError = {
    message: "Password doesn't match",
};

const UnexpectedError: FormError = {
    message: "An unexpected error occured.",
};

const empty = "\u200b";

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
    const [error, setError] = useState<FormError | null>(null);
    const auth = useAuthContextData();
    const { setAuthenticated } = useAuthContextActions();
    let isSignUp = form.confirmedPassword !== undefined;

    const submit = (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        if (isSignUp && form.password.value !== form.confirmedPassword?.value) {
            setError(PasswordNotMatchingError);
            return;
        }

        const endpoint = isSignUp ? "signup" : "login";

        axios
            .post<AuthRequest>(`http://127.0.0.1:8080/api/auth/${endpoint}`, {
                username: form.email.value,
                password: form.password.value,
            })
            .then((result) => {
                setAuthenticated();
                setError(null);
                console.log(result);
            })
            .catch((error) => {
                switch (error.response.status) {
                    case 401:
                        setError(InvalidCredentialError);
                        break;
                    case 409:
                        setError(DuplicateUserError);
                        break;
                    default:
                        setError(UnexpectedError);
                }
            });
    };

    const changeForm = () => {
        if (isSignUp) {
            let newForm = { ...form };
            delete newForm.confirmedPassword;
            setForm(newForm);
        } else {
            setForm((prev) => ({
                ...prev,
                confirmedPassword: baseSignUpForm.confirmedPassword,
            }));
        }
        setError(null);
    };

    const formFields = Object.keys(form) as Array<keyof CredentialForm>;

    return (
        <dialog
            open={auth === AuthState.AuthInProgress}
            className="p-4 rounded-xl bg-green-secondary"
        >
            <div className="flex-col min-h-96 w-96">
                <form
                    method="dialog"
                    onSubmit={submit}
                    className="flex-col justify-center"
                >
                    {formFields.map((key) => (
                        <div key={key} className="relative pb-4">
                            <input
                                id={key}
                                className="block rounded-lg px-2 pb-3 pt-5 w-full text-gray-900 bg-gray-50 appearance-none focus:outline-none focus:ring-0 peer"
                                placeholder=""
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
                            <label
                                htmlFor={key}
                                className="absolute text-gray-500 duration-300 transform -translate-y-4 scale-75 top-4 z-10 origin-[0] start-2.5 peer-placeholder-shown:scale-100 peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-4 rtl:peer-focus:translate-x-1/4 rtl:peer-focus:left-auto"
                            >
                                {form[key]?.label}
                            </label>
                        </div>
                    ))}
                    <div className="pb-4 flex justify-center">
                        <span className="text-yellow-primary">
                            {error?.message ?? empty}
                        </span>
                    </div>

                    <div className="pb-4">
                        <button
                            type="submit"
                            className="py-4 w-full rounded-lg duration-200 bg-yellow-primary hover:bg-orange-primary shadow-sm hover:shadow-md"
                        >
                            <span className="inline-block align-middle text-green-primary">
                                {isSignUp ? "Create account" : "Sign in"}
                            </span>
                        </button>
                    </div>
                </form>
                <div className="flex justify-center">
                    <button onClick={changeForm} className="">
                        <span className="inline-block align-middle text-white hover:text-yellow-primary duration-200">
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
