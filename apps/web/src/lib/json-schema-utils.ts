import set from "lodash/set";

/*
📣 These utils will be part of json-schema-form soon
*/

/**
 * Convert Form values to JSON values
 * Otherwise it will cause unexpected errors, such as
 * - number fields: { age: "5" } -> The value "5" must be a number.
 * - empty number fields: { age: "" } -> The value "" must be a number.
 * etc....
 */
export function formValuesToJsonValues(fields: any, values: any) {
    console.log("[JSON SCHEMA UTILS] Starting form values conversion", { fields, values });
    
    const fieldTypeTransform: any = {
        number: (val: any) => (val === "" ? val : +val),
        text: (val: any) => val,
        select: (val: any) => val,
        object_or_variable: (val: any) => {
            //This allows 
            // Handle variable pattern by wrapping it in an object
            if (typeof val === "string" && /^{{.*}}$/.test(val.trim())) {
                console.log("[JSON SCHEMA UTILS] Detected variable pattern, wrapping in object", { val });
                return { variable: val };
            }
            // Handle regular JSON objects
            try {
                const parsed = typeof val === "string" ? JSON.parse(val) : val;
                console.log("[JSON SCHEMA UTILS] Successfully parsed JSON object", { val, parsed });
                return parsed;
            } catch {
                console.log("[JSON SCHEMA UTILS] Failed to parse JSON, returning original value", { val });
                return val;
            }
        }
    };

    const jsonValues = {};

    fields.forEach(({ name, type, isVisible }: any) => {
        const formValue = values[name];
        const transformedValue: any = fieldTypeTransform[type]?.(formValue) || formValue;
        console.log("[JSON SCHEMA UTILS] Processing field", { name, type, isVisible, formValue, transformedValue });

        if (transformedValue === "") {
            console.log("[JSON SCHEMA UTILS] Omitting empty field to avoid type error", { name });
            // Omit empty fields from payload to avoid type error.
            // eg { team_size: "" } -> The value ("") must be a number.
        } else if (!isVisible) {
            console.log("[JSON SCHEMA UTILS] Omitting invisible conditional field", { name });
            // Omit invisible (conditional) fields to avoid erro:
            // eg { account: "personal", team_size: 3 } -> The "team_size" is invalid

        } else {
            console.log("[JSON SCHEMA UTILS] Setting field value", { name, transformedValue });
            set(jsonValues, name, transformedValue);
        }
    });

    console.log("[JSON SCHEMA UTILS] Finished converting form values", { jsonValues });
    return jsonValues;
}

/**
 * Set the initial values for the UI (controlled) components
 * based on the JSON Schema structure ("default" key) or arbitatry initialValues
 */
export function getDefaultValuesFromFields(fields: any, initialValues: any) {
    // TODO/BUG needs to support fieldsets recursively
    // console.log("fields", fields);
    // console.log("initialValues", initialValues);
    
    if (!Array.isArray(fields)) {
        return {};
    }

    return fields.reduce((acc: any, cur: any) => {
        return {
            ...acc,
            [cur.name]: (initialValues || {})[cur.name] || cur.default || ""
        };
    }, {});
}
