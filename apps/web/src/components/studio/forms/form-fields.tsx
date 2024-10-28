import { useState, useEffect, useRef } from "react";
// import Prism from "prismjs";
// import "prismjs/themes/prism.css";

import { Label } from "@repo/ui/components/ui/label";

import { ExpandableInput } from "@repo/ui/components/ui/expandable-input";
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

// Create a custom grammar for our template syntax
// Prism.languages.template = {
//   variable: {
//     pattern: /{{[\w.]+}}/g,
//     inside: {
//       punctuation: /{{|}}/,
//       "variable-name": /[\w.]+/,
//     },
//   },
// };

// Create a custom grammar for our template syntax
// Prism.languages.template = {
//   variable: {
//     pattern: /{{[\w.]+}}/g,
//     inside: {
//       punctuation: /{{|}}/,
//       "variable-name": /[\w.]+/,
//     },
//   },
// };

// import { useEffect, useRef, useState } from 'react';

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
  const contentRef = useRef<HTMLDivElement>(null); // Ref to the contenteditable div

  if (!isVisible) {
    console.log("fieldtext not visible", name);
    return null;
  }

  console.log("[RENDERING TEXT FIELD: ", name, " = ", value, "]");

  // Function to highlight syntax inside contenteditable
  function highlightSyntax(text: string) {
    return text.replace(/(\{\{.*?\}\})/g, '<span class="highlight">$1</span>');
  }

  // Sync the content back to the onChange handler
  function syncContent() {
    const rawValue = contentRef.current?.innerText || "";
    onChange(name, rawValue); // Call onChange with the raw text
  }

  // Handle input changes and apply syntax highlighting
  function handleInput() {
    if (!touched) setTouched(true);
    syncContent(); // Sync raw content
    applyHighlighting(); // Reapply highlighting
  }

  // Apply syntax highlighting with cursor preservation
  function applyHighlighting() {
    const rawText = contentRef.current?.innerText || "";
    const highlighted = highlightSyntax(rawText);
    contentRef.current!.innerHTML = highlighted;
    placeCursorAtEnd(contentRef.current!); // Keep cursor in place
  }

  // Keep cursor at the end after updating the innerHTML
  function placeCursorAtEnd(el: HTMLElement) {
    const range = document.createRange();
    const sel = window.getSelection();
    range.selectNodeContents(el);
    range.collapse(false);
    sel?.removeAllRanges();
    sel?.addRange(range);
  }

  // Initialize with syntax highlighting on mount
  useEffect(() => {
    if (contentRef.current) {
      contentRef.current.innerText = value || "";
      applyHighlighting(); // Initial highlighting
    }
  }, [value]);

  return (
    <div className="grid gap-3 my-4">
      <Label htmlFor={name}>{label}</Label>
      <div
        ref={contentRef}
        contentEditable
        className="editable-input border border-gray-300 p-2 rounded"
        onInput={handleInput}
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
      <style jsx global>{`
        .editable-input {
          white-space: pre-wrap;
          font-family: monospace;
          min-height: 100px;
          background-color: #fafafa;
          line-height: 1.5;
        }

        .highlight {
          background-color: #e2f1ff;
          border-radius: 3px;
          padding: 0 2px;
          border: 1px solid #b3d4ff;
          margin: 0 1px;
          display: inline-block;
        }

        /* Ensure text is visible */
        .editable-input,
        .editable-input * {
          color: #000;
        }
      `}</style>
    </div>
  );
}

// export default FieldText;

// function FieldText({
//   type,
//   name,
//   label,
//   const: constantValue,
//   default: defaultValue,
//   description,
//   value,
//   isVisible,
//   error,
//   submited,
//   onChange,
//   required,
//   ...props
// }: any) {
//   const [touched, setTouched] = useState(false);

//   if (!isVisible) return null;

//   function handleChange(e: any) {
//     if (!touched) setTouched(true);
//     onChange(name, e.target.value);
//   }
//   const highlightedValue = value
//     ? Prism.highlight(
//         value,
//         Prism.languages.template as Prism.Grammar,
//         "template",
//       )
//     : "";

//   return (
//     <div className="grid gap-3 my-4">
//       <Label htmlFor={name}>{label}</Label>
//       <div className="relative">
//         <ExpandableInput
//           id={name}
//           disabled={
//             constantValue && defaultValue && constantValue === defaultValue
//           }
//           value={value}
//           onChange={handleChange}
//           aria-invalid={!!error}
//           aria-describedby={`${name}-error ${name - description}`}
//           aria-required={required}
//           className="font-mono text-transparent caret-black selection:bg-blue-100 whitespace-pre-wrap break-words"
//           {...props}
//         />
//         <div
//           aria-hidden="true"
//           className="absolute top-0 left-0 right-0 bottom-0 pointer-events-none px-3 py-2 text-sm font-mono whitespace-pre-wrap break-words overflow-hidden"
//           dangerouslySetInnerHTML={{ __html: highlightedValue }}
//         />
//       </div>
//       <style jsx>{`
//         /* Custom styles for our template syntax */
//         :global(.token.variable) {
//           background: #dbeafe;
//           border-radius: 3px;
//           color: #3b82f6;
//           display: inline;
//           white-space: pre-wrap;
//           word-break: break-word;
//         }
//         :global(.token.punctuation) {
//           display: none; /* Hide {{ and }} */
//         }
//         :global(.token.variable-name) {
//           color: #3b82f6;
//         }
//         :global(.token) {
//           white-space: pre-wrap;
//           word-break: break-word;
//         }
//       `}</style>
//       {(touched || submited) && error && (
//         <div className="text-red-500" id={`${name}-error`}>
//           {error}
//         </div>
//       )}
//     </div>
//   );
// }
// best we got of the hard versino now trying different
// function FieldText({
//   type,
//   name,
//   label,
//   const: constantValue,
//   default: defaultValue,
//   description,
//   value,
//   isVisible,
//   error,
//   submited,
//   onChange,
//   required,
//   ...props
// }: any) {
//   const [touched, setTouched] = useState(false);

//   if (!isVisible) return null;

//   // Helper function to parse template variables
//   const parseTemplateVariables = (text: string) => {
//     const regex = /{{([\w.]+)}}/g;
//     const parts = [];
//     let lastIndex = 0;
//     let match;

//     while ((match = regex.exec(text)) !== null) {
//       // Add text before the variable
//       if (match.index > lastIndex) {
//         parts.push({
//           type: "text",
//           content: text.slice(lastIndex, match.index),
//         });
//       }

//       // Add the variable
//       const variablePath = match[1];
//       const variableName = variablePath?.split(".").pop() || variablePath;
//       parts.push({
//         type: "variable",
//         content: match[0],
//         displayName: variableName,
//       });

//       lastIndex = regex.lastIndex;
//     }

//     // Add remaining text
//     if (text && lastIndex < text.length) {
//       parts.push({
//         type: "text",
//         content: text.slice(lastIndex),
//       });
//     }

//     return parts;
//   };

//   function handleChange(e: any) {
//     if (!touched) setTouched(true);
//     onChange(name, e.target.value);
//   }

//   return (
//     <div className="grid gap-3 my-4">
//       <Label htmlFor={name}>{label}</Label>
//       <div className="relative">
//         <ExpandableInput
//           id={name}
//           type="text"
//           disabled={
//             constantValue && defaultValue && constantValue === defaultValue
//           }
//           value={value}
//           onChange={handleChange}
//           aria-invalid={!!error}
//           aria-describedby={`${name}-error ${name}-description`}
//           aria-required={required}
//           className="font-mono text-transparent selection:bg-blue-200 selection:text-transparent caret-black"
//           {...props}
//         />
//         <div
//           aria-hidden="true"
//           className="absolute top-0 left-0 right-0 bottom-0 pointer-events-none p-2 font-mono"
//         >
//           {parseTemplateVariables(value).map((part, index) =>
//             part.type === "variable" ? (
//               <span
//                 key={index}
//                 className="bg-blue-100 text-blue-800 px-1 rounded mx-0.5"
//               >
//                 {part.displayName}
//               </span>
//             ) : (
//               <span key={index} className="text-black">
//                 {part.content}
//               </span>
//             ),
//           )}
//         </div>
//       </div>
//       {(touched || submited) && error && (
//         <div className="text-red-500" id={`${name}-error`}>
//           {error}
//         </div>
//       )}
//     </div>
//   );
// }

// function FieldText({
//   type,
//   name,
//   label,
//   const: constantValue,
//   default: defaultValue,
//   description,
//   value,
//   isVisible,
//   error,
//   submited,
//   onChange,
//   required,
//   ...props
// }: any) {
//   const [touched, setTouched] = useState(false);

//   if (!isVisible) return null;

//   // Helper function to parse template variables
//   const parseTemplateVariables = (text: string) => {
//     const regex = /{{([\w.]+)}}/g;
//     const parts = [];
//     let lastIndex = 0;
//     let match;

//     while ((match = regex.exec(text)) !== null) {
//       // Add text before the variable
//       if (match.index > lastIndex) {
//         parts.push({
//           type: "text",
//           content: text.slice(lastIndex, match.index),
//         });
//       }

//       // Add the variable
//       const variablePath = match[1];
//       const variableName = variablePath?.split(".").pop() || variablePath;
//       parts.push({
//         type: "variable",
//         content: match[0],
//         displayName: variableName,
//       });

//       lastIndex = regex.lastIndex;
//     }

//     // Add remaining text
//     if (lastIndex < text.length) {
//       parts.push({
//         type: "text",
//         content: text.slice(lastIndex),
//       });
//     }

//     return parts;
//   };

//   function handleChange(e: any) {
//     if (!touched) setTouched(true);
//     onChange(name, e.target.value);
//   }

//   function handleKeyDown(e: React.KeyboardEvent) {
//     // Handle backspace for variables
//     if (e.key === "Backspace") {
//       const input = e.target as HTMLInputElement;
//       const cursorPosition = input.selectionStart || 0;
//       const value = input.value;

//       // Check if cursor is at the end of or within a variable
//       const regex = /{{[\w.]+}}/g;
//       let match;
//       while ((match = regex.exec(value)) !== null) {
//         const varStart = match.index;
//         const varEnd = varStart + match[0].length;

//         if (cursorPosition >= varStart && cursorPosition <= varEnd) {
//           // Prevent default backspace
//           e.preventDefault();
//           // Remove entire variable
//           const newValue = value.slice(0, varStart) + value.slice(varEnd);
//           onChange(name, newValue);
//           // Set cursor position
//           setTimeout(() => input.setSelectionRange(varStart, varStart));
//           break;
//         }
//       }
//     }
//   }

//   const renderContent = () => {
//     if (!value) return null;

//     const parts = parseTemplateVariables(value);

//     return parts.map((part, index) => {
//       if (part.type === "variable") {
//         return (
//           <span
//             key={index}
//             className="bg-blue-100 text-blue-800 px-1 rounded mx-0.5 font-mono"
//           >
//             {part.displayName}
//           </span>
//         );
//       }
//       return <span key={index}>{part.content}</span>;
//     });
//   };

//   return (
//     <div className="grid gap-3 my-4">
//       <Label htmlFor={name}>{label}</Label>
//       <div className="relative">
//         <ExpandableInput
//           id={name}
//           type="text"
//           disabled={
//             constantValue && defaultValue && constantValue === defaultValue
//           }
//           defaultValue={value}
//           onChange={handleChange}
//           onKeyDown={handleKeyDown}
//           aria-invalid={!!error}
//           aria-describedby={`${name}-error ${name}-description`}
//           aria-required={required}
//           {...props}
//         />
//         {value && (
//           <div className="absolute top-0 left-0 right-0 bottom-0 pointer-events-none p-2">
//             {renderContent()}
//           </div>
//         )}
//       </div>
//       {(touched || submited) && error && (
//         <div className="text-red-500" id={`${name}-error`}>
//           {error}
//         </div>
//       )}
//     </div>
//   );
// }
// function FieldText({
//   type,
//   name,
//   label,
//   const: constantValue,
//   default: defaultValue,
//   description,
//   value,
//   isVisible,
//   error,
//   submited,
//   onChange,
//   required,
//   ...props
// }: any) {
//   const [touched, setTouched] = useState(false);

//   if (!isVisible) {
//     console.log("fieldtext not visible", name);
//     return null;
//   }

//   console.log("[RENDERING TEXT FIELD: ", name, " = ", value, "]");

//   function handleChange(e: any) {
//     console.log("fieldtext handleChange: ", e);
//     if (!touched) setTouched(true);
//     onChange(name, e.target.value);
//   }

//   return (
//     <div className="grid gap-3 my-4">
//       <Label htmlFor={name}>{label}</Label>
//       {/* {description && <div id={`${name}-description`}>{description}</div>} */}
//       <ExpandableInput
//         id={name}
//         type="text"
//         disabled={
//           constantValue && defaultValue && constantValue === defaultValue
//         }
//         defaultValue={value}
//         onChange={handleChange}
//         aria-invalid={!!error}
//         aria-describedby={`${name}-error ${name}-description`}
//         aria-required={required}
//         {...props}
//       />
//       {(touched || submited) && error && (
//         <div className="text-red-500" id={`${name}-error`}>
//           {error}
//         </div>
//       )}
//     </div>
//   );
// }

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
