import { useState, useEffect } from "react";

import { Label } from "@repo/ui/components/ui/label";
import { Input } from "@repo/ui/components/ui/input";
import { Checkbox } from "@repo/ui/components/ui/checkbox";
import {
  Select,
  SelectTrigger,
  SelectValue,
  SelectContent,
  SelectItem,
} from "@repo/ui/components/ui/select";

import api from "@repo/anything-api";
import { BaseNodeIcon, BaseSelectIcon } from "../nodes/node-icon";
import { Button } from "@repo/ui/components/ui/button";
import Link from "next/link";
import { useAnything } from "@/context/AnythingContext";

export const fieldsMap: { [key: string]: any } = {
  text: FieldText,
  account: FieldAccount,
  number: FieldNumber,
  radio: FieldRadio,
  select: FieldSelect,
  checkbox: FieldCheckbox,
  error: FieldUnknown,
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

  if (!isVisible) {
    console.log("fieldtext not visible", name);
    return null;
  }

  console.log("[RENDERING TEXT FIELD: ", name, " = ", value, "]");

  function handleChange(e: any) {
    console.log("fieldtext handleChange: ", e);
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
        disabled={
          constantValue && defaultValue && constantValue === defaultValue
        }
        defaultValue={value}
        onChange={handleChange}
        aria-invalid={!!error}
        aria-describedby={`${name}-error ${name}-description`}
        aria-required={required}
        {...props}
      />
      {(touched || submited) && error && (
        <div className="text-red-500" id={`${name}-error`}>
          {error}
        </div>
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
  onChange,
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
      <Label aria-label={`${label} ${displayError}`}>{label}</Label>
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
      {displayError && <div className="text-red-500">{displayError}</div>}
    </fieldset>
  );
}

function FieldCheckbox({
  name,
  label,
  description,
  value,
  options,
  isVisible,
  error,
  submited,
  onChange,
}: any) {
  const [touched, setTouched] = useState(false);

  if (!isVisible) return null;

  function handleChange(e: any) {
    console.log("checkbox e", e);
    if (!touched) setTouched(true);
    onChange(name, e);
  }

  const displayError = submited || touched ? error : null;

  return (
    <div key={name} className="grid gap-3 my-4">
      {/* A11Y errors: https://blog.tenon.io/accessible-validation-of-checkbox-and-radiobutton-groups/ */}
      <Label htmlFor={name}>{label}</Label>

      <div className="flex items-center">
        <Checkbox name={name} checked={value} onCheckedChange={handleChange} />
        <label
          htmlFor={name}
          className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70"
        >
          {value ? (
            <span className="p-1 bg-green-400 rounded-lg ml-2">Active</span>
          ) : (
            <span className="p-1 bg-red-400 rounded-lg ml-2">Inactive</span>
          )}
        </label>
        {(touched || submited) && error && (
          <div className="text-red-500" id={`${name}-error`}>
            {error}
          </div>
        )}
      </div>
    </div>
  );
}

function FieldUnknown({ type, name, error }: any) {
  return (
    <p style={{ border: "1px dashed gray", padding: "8px" }}>
      Field "{name}" unsupported: The type "{type}" has no UI component built
      yet.
      {error && (
        <div className="text-red-500" id={`${name}-error`}>
          {error}
        </div>
      )}
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

  function handleValueChange(value: any) {
    if (!touched) setTouched(true);
    // don't pass it to onchange if the value is an empty string
    if (value === "") return;
    onChange(name, value);
    // onChange(value); //THIS maybe caused like a 2 day bug. It should be onChange(name, e.target.value)? on onValueChange
  }

  console.log("[RENDERING SELECT FIELD: ", name, " = ", value, "]");
  console.log("[SELECT OPTIONS]", options);

  return (
    <div className="grid gap-3 my-4">
      <Label htmlFor={name}>{label}</Label>
      <Select value={value} onValueChange={handleValueChange}>
        <SelectTrigger>
          <SelectValue placeholder={description} />
        </SelectTrigger>
        <SelectContent>
          {options.map((option: any) => (
            <SelectItem key={option.value} value={option.value}>
              {option.label}
            </SelectItem>
          ))}
        </SelectContent>
      </Select>
      {(touched || submited) && error && (
        <div className="text-red-500" id={`${name}-error`}>
          {error}
        </div>
      )}
    </div>
  );
}

function FieldAccount({
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
  provider,
  ...props
}: any) {
  const [touched, setTouched] = useState(false);
  const [hydrated, setHydrated] = useState(false);
  const [accountsForProvider, setAccountsForProvider] = useState<any[any]>([]);
  const [providerDetails, setProviderDetails] = useState<any[any]>([]);
  const {
    accounts: { selectedAccount },
  } = useAnything();

  //TODO:
  //Load check if user has an account that matches the variable schema
  //If no matches show a "add an account button" for the user to do teh oauth flow
  //If the user has an account select teh first one automatically.

  //   console.log("Props in account", props);

  if (!isVisible) return null;

  function handleValueChange(e: any) {
    if (!touched) setTouched(true);
    onValueChange(e);
  }

  const getProviderDetails = async () => {
    try {
      console.log("provider in form field", provider);
      if (!selectedAccount) return;
      let res = await api.auth.getProvider(
        selectedAccount.account_id,
        provider,
      );
      console.log("res for getProviderDetails", res);
      setProviderDetails(res[0]);
    } catch (e) {
      console.log("error in getProvider");
    }
  };

  const getUserAccountsForProvider = async () => {
    try {
      if (!provider || !selectedAccount) return;
      let res = await api.auth.getAuthAccountsForProvider(
        selectedAccount?.account_id,
        provider,
      );
      console.log("res", res);
      setAccountsForProvider(res);
      setHydrated(true);
    } catch (e) {
      console.log("error in getUserAccounts");
    }
  };

  const connect = async (e: any) => {
    e.preventDefault();
    try {
      // let res = await api.auth.connectProvider(provider);
      // console.log("res", res);
    } catch (e) {
      console.log("error in connect");
    }
  };

  useEffect(() => {
    console.log("provider", provider);
    if (provider) {
      getProviderDetails();
      getUserAccountsForProvider();
    }
  }, []);

  return (
    <div className="grid gap-3 my-4">
      <Label htmlFor={name}>{label}</Label>
      {!hydrated ? (
        <div>Loading...</div>
      ) : (
        <>
          {providerDetails && accountsForProvider.length === 0 ? (
            <div className="flex flex-row items-center border rounded-md p-2">
              <BaseNodeIcon icon={providerDetails.provider_icon} />
              <div className="text-xl ml-2">Connect your Airtable Account</div>
              <div className="ml-auto">
                <Link href="/accounts">
                  <Button variant="outline">Add Account</Button>
                </Link>
                {/* <Button onClick={connect} variant="outline">
                  Connect
                </Button> */}
              </div>
            </div>
          ) : (
            <Select value={value} onValueChange={handleValueChange}>
              <SelectTrigger>
                <SelectValue placeholder={description} />
              </SelectTrigger>
              <SelectContent>
                {accountsForProvider.map((option: any) => (
                  <SelectItem
                    key={option.account_auth_provider_account_slug}
                    value={`{{accounts.${option.account_auth_provider_account_slug}}}`}
                  >
                    <div className="flex flex-row">
                      <div className="mr-2">
                        <BaseSelectIcon icon={providerDetails.provider_icon} />
                      </div>

                      <div className="text-lg flex items-center">
                        {option.account_auth_provider_account_label}
                      </div>
                    </div>
                  </SelectItem>
                ))}
                <div className="border-t my-2 mb-2" />
                <div className="">
                  <div className="flex flex-row">
                    <div className="mr-2 ml-8">
                      <BaseSelectIcon icon={providerDetails.provider_icon} />
                    </div>
                    <div className="text-lg flex items-center">
                      Connnect New Account
                    </div>
                    <div className="ml-auto">
                      <Link href="/accounts">
                        <Button variant="outline">Add Account</Button>
                      </Link>
                    </div>
                  </div>
                </div>
              </SelectContent>
            </Select>
          )}
        </>
      )}
      {/* <Label htmlFor={name}>{label}</Label> */}

      {(touched || submited) && error && (
        <div className="text-red-500" id={`${name}-error`}>
          {error}
        </div>
      )}
    </div>
  );
}
