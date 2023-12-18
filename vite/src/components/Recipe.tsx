import {
    useRecipeContextActions,
    useRecipeContextData,
} from "../context/RecipeContext";
import RecipeRow from "./RecipeRow";

function Recipe() {
    const rows = useRecipeContextData();
    const macros =
        rows.length > 0
            ? rows
                  .map((row) => ({
                      calories: row.calories,
                      protein: row.protein,
                      fat: row.fat,
                      carb: row.carb,
                  }))
                  .reduce((acc, row) => ({
                      calories: acc.calories + row.calories,
                      protein: acc.protein + row.protein,
                      fat: acc.fat + row.fat,
                      carb: acc.carb + row.carb,
                  }))
            : {
                  calories: 0,
                  protein: 0,
                  fat: 0,
                  carb: 0,
              };
    const { addRow } = useRecipeContextActions();

    return (
        <div>
            <button onClick={addRow}>{"Add ingredient"}</button>
            {rows.map((row) => (
                <RecipeRow key={`recipeRow-${row.rowId}`} row={row} />
            ))}
            <div>
                <span>{`Calories ${macros.calories.toFixed(0)}`}</span>
            </div>
            <div>
                <span>{`Protein ${macros.protein.toFixed(1)}g`}</span>
            </div>
            <div>
                <span>{`Fat ${macros.fat.toFixed(1)}g`}</span>
            </div>
            <div>
                <span>{`Carbs ${macros.carb.toFixed(1)}g`}</span>
            </div>
        </div>
    );
}

export default Recipe;
