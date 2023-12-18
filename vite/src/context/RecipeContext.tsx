import { ReactNode, createContext, useContext, useMemo, useState } from "react";

export type RecipeRowState = {
    rowId: number;
    ingredient: string;
    measure: string;
    ingredientId: number;
    measureId: number;
    conversionFactor: number;
    calories: number;
    protein: number;
    fat: number;
    carb: number;
};

const NEW_ROW: RecipeRowState = {
    rowId: -1,
    ingredient: "",
    measure: "",
    ingredientId: -1,
    measureId: -1,
    conversionFactor: 0,
    calories: 0,
    protein: 0,
    fat: 0,
    carb: 0,
};

type RecipeState = Array<RecipeRowState>;

type Actions = {
    addRow: () => void;
    updateRow: (row: RecipeRowState) => void;
    deleteRow: (rowId: number) => void;
};

const RecipeContextData = createContext<RecipeState>([]);
const RecipeContextActions = createContext<Actions>({} as Actions);

type Props = {
    children: ReactNode;
};

function RecipeContext({ children }: Props) {
    const [state, setState] = useState<RecipeState>([]);
    const actions = useMemo(
        () => ({
            addRow: () =>
                setState((prevState) => [
                    ...prevState,
                    { ...NEW_ROW, rowId: prevState.length },
                ]),
            updateRow: (row: RecipeRowState) =>
                setState((prevState) =>
                    prevState.map((r) => (r.rowId === row.rowId ? row : r))
                ),
            deleteRow: (rowId: number) =>
                setState((prevState) => {
                    let newRows = prevState.filter((r) => r.rowId !== rowId);
                    newRows.forEach((r, i) => (r.rowId = i));
                    return newRows;
                }),
        }),
        []
    );

    return (
        <RecipeContextActions.Provider value={actions}>
            <RecipeContextData.Provider value={state}>
                {children}
            </RecipeContextData.Provider>
        </RecipeContextActions.Provider>
    );
}

export const useRecipeContextData = () => useContext(RecipeContextData);
export const useRecipeContextActions = () => useContext(RecipeContextActions);

export default RecipeContext;
