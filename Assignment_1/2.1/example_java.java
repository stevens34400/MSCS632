public class MemoryManagementExample {
    public static void main(String[] args) {
        // Dynamically allocate an array on the heap.
        int[] arr = new int[10];
        for (int i = 0; i < arr.length; i++) {
            arr[i] = i * 10;
        }
        System.out.println("Array values:");
        for (int val : arr) {
            System.out.print(val + " ");
        }
        System.out.println();

        // Once main() exits and there are no more references to 'arr',
        // Basically allowed to be automatically garbage collected.
    }
}
