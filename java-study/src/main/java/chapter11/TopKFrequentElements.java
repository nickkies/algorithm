package chapter11;

import java.util.Arrays;
import java.util.Comparator;
import java.util.Map;
import java.util.PriorityQueue;
import java.util.function.Function;
import java.util.stream.Collectors;

public class TopKFrequentElements {
    public static int[] solution(int[] nums, int k) {
        Map<Integer, Long> map = Arrays.stream(nums)
                .boxed()
                .collect(Collectors.groupingBy(Function.identity(), Collectors.counting()));

        PriorityQueue<Map.Entry<Integer, Long>> minHeap = new PriorityQueue<>(Comparator.comparingLong(Map.Entry::getValue));

        map
            .entrySet()
            .forEach(entry -> {
                minHeap.offer(entry);
                if (minHeap.size() > k) minHeap.poll();
            });

        return minHeap.stream()
                .mapToInt(Map.Entry::getKey)
                .toArray();
    }
}
