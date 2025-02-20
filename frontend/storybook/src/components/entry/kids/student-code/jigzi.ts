import {argsToAttrs} from "@utils/attributes";
import "@elements/entry/kids/student-code/jigzi";
import { Mode } from "@elements/entry/home/student-code/jigzi";

export default {
    title: "Entry / Kids / Student Code"
}

interface Args {
    mode: Mode,
}

const DEFAULT_ARGS:Args = {
    mode: "default",
}

export const Jigzi = (props?:Args) => {

    props = props ? {...DEFAULT_ARGS, ...props} : DEFAULT_ARGS;

    return `
        <kids-student-code-jigzi slot="jigzi" ${argsToAttrs(props)}>
            <button slot="help" onclick="console.log('help)">Ask for help</button>
            <button slot="try-again" onclick="console.log('try again')">Try again</button>
        </kids-student-code-jigzi>
    `;
}

Jigzi.args = DEFAULT_ARGS;
Jigzi.argTypes = {
    mode: {
        control: {
            type: 'inline-radio',
            options: ["default", "try-again", "help"]
        }
    }
}