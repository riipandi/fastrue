export function urlBase64Decode(str: string) {
  // From https://jwt.io/js/jwt.js
  let output = str.replace(/-/g, '+').replace(/_/g, '/')
  switch (output.length % 4) {
    case 0:
      break
    case 2:
      output += '=='
      break
    case 3:
      output += '='
      break
    default:
      throw 'Illegal base64url string!'
  }

  // polifyll https://github.com/davidchambers/Base64.js
  const result = window.atob(output)
  try {
    return decodeURIComponent(escape(result))
  } catch {
    return result
  }
}
