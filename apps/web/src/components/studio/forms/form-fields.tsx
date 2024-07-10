import { useState } from "react";

import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import { Checkbox } from "@/components/ui/checkbox";
import { Select, SelectTrigger, SelectValue, SelectContent, SelectItem } from "@/components/ui/select";


export const fieldsMap: { [key: string]: any } = {
    text: FieldText,
    number: FieldNumber,
    radio: FieldRadio,
    select: FieldSelect,
    error: FieldUnknown
};

function FieldText({
    type,
    name,
    label,
    const: constantValue,
    default: defaultValue,
    description,
    value,
    isVisible,
    error,
    submited,
    onChange,
    required,
    ...props
}: any) {
    const [touched, setTouched] = useState(false);

    if (!isVisible) return null;

    // console.log("fieldtext name", label)
    // // console.log("fieldtext props", props)
    // console.log("fieldtext const", constantValue)
    // console.log("fieldtext default", defaultValue)

    function handleChange(e: any) {
        if (!touched) setTouched(true);
        onChange(name, e.target.value);
    }

    return (
        <div className="grid gap-3 my-4">
            <Label htmlFor={name}>{label}</Label>
            {/* {description && <div id={`${name}-description`}>{description}</div>} */}
            <Input
                id={name}
                type="text"
                disabled={constantValue && defaultValue && constantValue === defaultValue}
                defaultValue={value}
                onChange={handleChange}
                aria-invalid={!!error}
                aria-describedby={`${name}-error ${name}-description`}
                aria-required={required}
                {...props}
            />
            {(touched || submited) && error && (
                <div id={`${name}-error`}>{error}</div>
            )}
        </div>
    );
}

function FieldNumber(props: any) {
    return (
        <FieldText
            inputMode="decimal"
            // accepts numbers and dots (eg 10, 15.50)
            pattern="^[0-9.]*$"
            {...props}
        />
    );
}

function FieldRadio({
    name,
    label,
    description,
    value,
    options,
    isVisible,
    error,
    submited,
    onChange
}: any) {
    const [touched, setTouched] = useState(false);

    if (!isVisible) return null;

    function handleChange(e: any) {
        if (!touched) setTouched(true);
        onChange(name, e.target.value);
    }

    const displayError = submited || touched ? error : null;

    return (
        <fieldset key={name}>
            {/* A11Y errors: https://blog.tenon.io/accessible-validation-of-checkbox-and-radiobutton-groups/ */}
            <Label aria-label={`${label} ${displayError}`}>
                {label}
            </Label>
            {description && <div>{description}</div>}
            <div onChange={handleChange}>
                {options.map((opt: any) => (
                    <Checkbox key={opt.value}>
                        <input
                            type="radio"
                            name={name}
                            value={opt.value}
                            defaultChecked={value === opt.value}
                        />
                        {opt.label}
                    </Checkbox>
                ))}
            </div>
            {displayError && <div>{displayError}</div>}
        </fieldset>
    );
}

function FieldUnknown({ type, name, error }: any) {
    return (
        <p style={{ border: "1px dashed gray", padding: "8px" }}>
            Field "{name}" unsupported: The type "{type}" has no UI component built
            yet.
            {error && <div id={`${name}-error`}>{error}</div>}
        </p>
    );
}

function FieldSelect({
    type,
    name,
    label,
    options,
    description,
    value,
    isVisible,
    error,
    submited,
    onChange,
    onValueChange,
    required,
    ...props
}: any) {
    const [touched, setTouched] = useState(false);

    if (!isVisible) return null;

    function handleValueChange(e: any) {
        if (!touched) setTouched(true);
        onValueChange(e);
    }

    return (
        <div className="grid gap-3 my-4">
            <Label htmlFor={name}>{label}</Label>
            <Select value={value} onValueChange={handleValueChange}>
                <SelectTrigger>
                    <SelectValue placeholder={description} />
                </SelectTrigger>
                <SelectContent>
                    {options.map((option: any) => (
                        <SelectItem key={option.label} value={option.value}>
                            {option.label}
                        </SelectItem>
                    ))}
                </SelectContent>
            </Select>
            {(touched || submited) && error && (
                <div id={`${name}-error`}>{error}</div>
            )}
        </div>
    );
}