import java.lang.reflect.Method;
import java.util.Map;
import java.util.function.Consumer;
import jdk.jshell.JShell;
import jdk.jshell.execution.DirectExecutionControl;
import jdk.jshell.spi.ExecutionControl;
import jdk.jshell.spi.ExecutionControlProvider;
import jdk.jshell.spi.ExecutionEnv;

class CustomExecutionControl extends DirectExecutionControl {
    Consumer<Object> consumer;
    CustomExecutionControl(Consumer<Object> consumer) {
        this.consumer = consumer;
    }

    @Override
    protected String invoke(Method doitMethod) throws Exception {
        return valueString(invokeRaw(doitMethod));
    }

    Object invokeRaw(Method doitMethod) throws Exception {
        Object res = doitMethod.invoke(null, new Object[0]);
        consumer.accept(res);
        return res;
    }
}

class CustomExecutionControlProvider implements ExecutionControlProvider {
    Consumer<Object> consumer;

    CustomExecutionControlProvider(Consumer<Object> consumer) {
        this.consumer = consumer;
    }

    @Override
    public String name() {
        return "test";
    }

    @Override
    public ExecutionControl generate(ExecutionEnv env, Map<String, String> parameters)
        throws Throwable {
        return new CustomExecutionControl(consumer);
    }
}

public class JShellFix {
    private JShell shell;
    private Object[] result = new Object[1];

    public JShellFix() {
        shell = JShell.builder()
            .executionEngine(new CustomExecutionControlProvider(val -> result[0] = val), null)
            .build();
    }

    public Object eval(String code) {
        result[0] = null;
        shell.eval(code);
        return result[0];
    }
}