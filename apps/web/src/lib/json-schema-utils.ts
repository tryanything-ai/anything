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
    
    const fieldTypeTransform: any = {
        number: (val: any) => (val === "" ? val : +val),
        text: (val: any) => val,
        select: (val: any) => val
        // TODO support all types
    };

    const jsonValues = {};

    fields.forEach(({ name, type, isVisible }: any) => {
        const formValue = values[name];
        const transformedValue: any = fieldTypeTransform[type]?.(formValue) || formValue;

        if (transformedValue === "") {
            // Omit empty fields from payload to avoid type error.
            // eg { team_size: "" } -> The value ("") must be a number.
        } else if (!isVisible) {
            // Omit invisible (conditional) fields to avoid erro:
            // eg { account: "personal", team_size: 3 } -> The "team_size" is invalid

        } else {
            set(jsonValues, name, transformedValue);
        }
    });

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
