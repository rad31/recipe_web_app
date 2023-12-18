import AuthContext from "./context/AuthContext";
import TopMenu from "./components/TopMenu";
import RecipeContext from "./context/RecipeContext";

function App() {
    return (
        <>
            <AuthContext>
                <RecipeContext>
                    <TopMenu />
                </RecipeContext>
            </AuthContext>
        </>
    );
}

export default App;
