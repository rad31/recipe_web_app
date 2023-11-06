import AuthContext from "./context/AuthContext";
import TopMenu from "./components/TopMenu";

function App() {
    return (
        <>
            <AuthContext>
                <TopMenu />
            </AuthContext>
        </>
    );
}

export default App;
