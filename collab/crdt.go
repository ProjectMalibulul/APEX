package collab

import "sort"

// ORSet is a deterministic observed-remove set for collaboration presence.
type ORSet struct {
	adds    map[string]map[string]struct{}
	removes map[string]map[string]struct{}
}

// NewORSet creates an empty observed-remove set.
func NewORSet() *ORSet {
	return &ORSet{
		adds:    map[string]map[string]struct{}{},
		removes: map[string]map[string]struct{}{},
	}
}

// Add records an add operation with a stable operation id.
func (s *ORSet) Add(value string, opID string) {
	if s.adds[value] == nil {
		s.adds[value] = map[string]struct{}{}
	}
	s.adds[value][opID] = struct{}{}
}

// Remove records a remove operation for every observed add operation.
func (s *ORSet) Remove(value string) {
	if s.removes[value] == nil {
		s.removes[value] = map[string]struct{}{}
	}
	for opID := range s.adds[value] {
		s.removes[value][opID] = struct{}{}
	}
}

// Values returns the current set in deterministic order.
func (s *ORSet) Values() []string {
	values := []string{}
	for value, opIDs := range s.adds {
		for opID := range opIDs {
			if _, removed := s.removes[value][opID]; !removed {
				values = append(values, value)
				break
			}
		}
	}
	sort.Strings(values)
	return values
}

// Merge joins another set into this set.
func (s *ORSet) Merge(other *ORSet) {
	for value, opIDs := range other.adds {
		for opID := range opIDs {
			s.Add(value, opID)
		}
	}
	for value, opIDs := range other.removes {
		if s.removes[value] == nil {
			s.removes[value] = map[string]struct{}{}
		}
		for opID := range opIDs {
			s.removes[value][opID] = struct{}{}
		}
	}
}
