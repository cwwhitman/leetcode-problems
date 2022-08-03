struct ListNode* mergeTwoLists(struct ListNode* list1, struct ListNode* list2) __attribute__ ((optimize("3")));

struct ListNode* mergeTwoLists(struct ListNode* list1, struct ListNode* list2) {
    if (!list1) return list2;
    
    if (list2 && list1->val > list2->val) {
        struct ListNode* temp = list1;
        list1 = list2;
        list2 = temp;
    }
    
    struct ListNode* head = list1;
    
    while (list2) {
        while (list1->next && list1->next->val < list2->val) {
            list1 = list1->next;
        }
        struct ListNode* temp = list1->next;
        list1->next = list2;
        list2 = list2->next;
        list1->next->next = temp;
    }

    return head;
}