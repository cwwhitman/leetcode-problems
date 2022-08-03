bool hasCycle(struct ListNode *head) {
    if (!head) return false;
    
    head->val = 1000000;
    while (head->next) {
        head = head->next;
        if (head->val == 1000000) {
            return true;
        } else {
            head->val = 1000000;
        }
    }
    
    return false;
}