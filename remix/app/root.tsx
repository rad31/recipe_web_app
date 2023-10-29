import { Links, LiveReload } from "@remix-run/react";
import TopMenu from "./components/TopMenu";
import stylesheet from "~/tailwind.css";
import { LinksFunction } from "@remix-run/react/dist/routeModules";

export const links: LinksFunction = () => [
  { rel: "stylesheet", href: stylesheet },
];

export default function App() {
  return (
    <html lang="en">
      <head>
        <meta charSet="utf-8" />
        <meta
          name="viewport"
          content="width=device-width, initial-scale=1"
        />
        <title>Remix: So great, it's funny!</title>
        <Links />
      </head>
      <body>
        <TopMenu />
        Hello world!!!
        <LiveReload />
      </body>
    </html>
  );
}
