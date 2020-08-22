addEventListener('fetch', event => {
  event.respondWith(handleRequest(event.request))
})

/**
 * Fetch and log a request
 * @param {Request} request
 */
async function handleRequest(request) {
  try {
    const json = JSON.parse(request.body),
      { computer } = wasm_bindgen;
    await wasm_bindgen(wasm);
    return new Response(computer(json), {
      status: 200,
      headers: { "Content-Type": 'application/json; charset="UTF-8"' }
    });
  } catch (e) {
    return Response.redirect("https://yuji.ne.jp/404.html");
  };
}
