package chapter11;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Test;

public class DesignHashMapTest {
    @Test
    public void test() {
        DesignHashMap map = new DesignHashMap();

        map.put(1, 1);
        map.put(2, 2);

        Assertions.assertEquals(map.get(1), 1);
        Assertions.assertEquals(map.get(3), -1);
        Assertions.assertEquals(map.get(2), 2);

        map.put(2, 1);

        Assertions.assertEquals(map.get(2), 1);

        map.remove(2);

        Assertions.assertEquals(map.get(2), -1);

    }
}
