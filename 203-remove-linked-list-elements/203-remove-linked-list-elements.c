struct ListNode* removeElements(struct ListNode* head, int val){
    struct ListNode dummy = {0, head};
    struct ListNode* node = &dummy;
    while (node && node->next) {
        while (node->next && node->next->val == val) {
            node->next = node->next->next;
        }
        node = node->next;
    }
    
    return dummy.next;
}