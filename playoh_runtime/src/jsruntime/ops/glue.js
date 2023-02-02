((window) => {
  async function fetch(args) {
    if (typeof args === "string") {
      args = { url: args, method: "GET", headers: [] };
    } else if (typeof args === "object") {
      if (args.url) {
        args.method = args.method || "GET";
        args.headers = args.headers || [];
        args.body = args.body || [];
      } else {
        throw new Error("url is required");
      }
    } else {
      throw new Error("args is required");
    }
    let res = await Deno.core.opAsync("op_fetch", args);
    res.text = () => {
      let body = res.body;
      if (!body) {
        return null;
      }
      return Deno.core.opSync("op_decode_utf8", body);
    };
    return res;
  }

  function email(msg) {
    if (typeof msg === "string") {
    let res =   Deno.core.opSync("op_send_email", msg);
   }
  }
  function result(inner_id,key,msg) {
    if (typeof msg === "string") {
    let args={"msg":msg,"id":inner_id,"key":key};
    Deno.core.opSync("op_add_msg", args);
   }
  }
  window.fetch = fetch;
  window.email = email;
  window.result =result;
})(this);
