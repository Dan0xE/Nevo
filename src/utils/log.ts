type message = "info" | "error";

/** Prints an error to the console and opens an alert with the specified message
 *
 *! THIS IS NOT THERE TO REPLACE BASIC CONSOLE.LOG STATEMENTS!
 */
export default function log(message: any, type: message): void {
  message === "info" ? console.log(...message) : console.error(message);
  alert(message);
}
