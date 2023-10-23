export const flowJsonFromBigFLow = (template: any) => {
    // TODO: this whole thing is kinda garbage and related to typescript problems with supabase queryes that are nested
    let flow_json: any;
  
    if (
      template.flow_template_versions &&
      Array.isArray(template.flow_template_versions)
    ) {
      flow_json = template.flow_template_versions[0].flow_template_json;
    } else {
      flow_json = null;
    }
  
    return flow_json;
  };
  