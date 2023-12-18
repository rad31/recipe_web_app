import { useCallback, useState } from "react";
import {
    RecipeRowState,
    useRecipeContextActions,
} from "../context/RecipeContext";
import axios from "axios";

type RecipeRowProps = {
    row: RecipeRowState;
};

type Food = {
    id: number;
    description: string;
};

type Measure = {
    id: number;
    name: string;
};

type Macros = {
    calories: number;
    protein: number;
    fat: number;
    carb: number;
};

function debounce(fn: Function, delayInMs = 300) {
    let timeoutId: ReturnType<typeof setTimeout>;
    return function (this: any, ...args: any[]) {
        clearTimeout(timeoutId);
        timeoutId = setTimeout(() => fn.apply(this, args), delayInMs);
    };
}

function RecipeRow({ row }: RecipeRowProps) {
    const [foods, setFoods] = useState<Array<Food>>([]);
    const [measures, setMeasures] = useState<Array<Measure>>([]);
    const [selectedFoodId, setSelectedFoodId] = useState<string | null>(null);
    const { updateRow } = useRecipeContextActions();

    const fetchFoods = useCallback(
        debounce((name: string) => {
            axios
                .get<Array<Food>>(
                    `http://127.0.0.1:8080/api/ingredients/food/${name}`
                )
                .then((res) => setFoods(res.data))
                .catch((err) => console.log(err));
        }),
        []
    );

    const fetchMeasures = (id: string) => {
        axios
            .get<Array<Measure>>(
                `http://127.0.0.1:8080/api/ingredients/measure/${id}`
            )
            .then((res) => setMeasures(res.data))
            .catch((err) => console.log(err));
    };

    const fetchMacros = (foodId: string, measureId: string) => {
        axios
            .get<Macros>(
                `http://127.0.0.1:8080/api/ingredients/macros/${foodId}/${measureId}`
            )
            .then((res) =>
                updateRow({
                    ...row,
                    calories: res.data.calories,
                    protein: res.data.protein,
                    fat: res.data.fat,
                    carb: res.data.carb,
                })
            )
            .catch((err) => console.log(err));
    };

    return (
        <div>
            <form className="flex pl-4 pt-4">
                <div className="relative pb-4 pr-2">
                    <input
                        id={`ingredient-${row.rowId}`}
                        className="block rounded-lg px-2 pb-3 pt-5 w-50 text-gray-900 bg-gray-50 appearance-none focus:outline-none focus:ring-0 peer"
                        type="text"
                        value={row.ingredient}
                        onChange={(e) => {
                            updateRow({
                                ...row,
                                ingredient: e.target.value,
                            });
                            if (e.target.value.length > 0) {
                                fetchFoods(e.target.value);
                            }
                        }}
                    />
                    <label
                        htmlFor={`ingredient-${row.rowId}`}
                        className="absolute text-gray-500 duration-300 transform -translate-y-4 scale-75 top-4 z-10 origin-[0] start-2.5 peer-shown:scale-100 peer-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-4 rtl:peer-focus:translate-x-1/4 rtl:peer-focus:left-auto"
                    >
                        {"Ingredient"}
                    </label>
                </div>
                <div className="relative pb-4 pr-2">
                    <input
                        id={`measure-${row.rowId}`}
                        className="block rounded-lg px-2 pb-3 pt-5 w-50 text-gray-900 bg-gray-50 appearance-none focus:outline-none focus:ring-0 peer"
                        type="text"
                        value={row.measure}
                        onChange={(e) => {
                            updateRow({
                                ...row,
                                measure: e.target.value,
                            });
                        }}
                    />
                    <label
                        htmlFor={`measure-${row.rowId}`}
                        className="absolute text-gray-500 duration-300 transform -translate-y-4 scale-75 top-4 z-10 origin-[0] start-2.5 peer-shown:scale-100 peer-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-4 rtl:peer-focus:translate-x-1/4 rtl:peer-focus:left-auto"
                    >
                        {"Amount"}
                    </label>
                </div>
                <div className="relative pb-4 pr-2">
                    <select
                        onChange={(e) => {
                            setMeasures([]);
                            setSelectedFoodId(e.target.value);
                            fetchMeasures(e.target.value);
                        }}
                        className="w-52"
                    >
                        {foods.map((food) => (
                            <option value={`${food.id}`} className="w-52">
                                {food.description}
                            </option>
                        ))}
                    </select>
                </div>
                <div className="relative pb-4 pr-2">
                    <select
                        className="w-52"
                        onChange={(e) => {
                            if (selectedFoodId !== null) {
                                fetchMacros(selectedFoodId, e.target.value);
                            }
                        }}
                    >
                        {measures.map((measure) => (
                            <option value={`${measure.id}`} className="w-52">
                                {measure.name}
                            </option>
                        ))}
                    </select>
                </div>
            </form>
        </div>
    );
}

export default RecipeRow;
